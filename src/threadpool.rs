use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

struct ThreadPoolThread<Arg>
where
    Arg: Send,
{
    arg_tx: Sender<(usize, Arg)>,
    _handle: JoinHandle<Result<(), ()>>,
}

pub struct ThreadPool<Arg, Res>
where
    Arg: Send,
    Res: Send,
{
    threads: Vec<ThreadPoolThread<Arg>>,
    res_rx: Receiver<(usize, Res)>,
}

impl<Arg, Res> ThreadPool<Arg, Res>
where
    Arg: Send + 'static,
    Res: Send + 'static,
{
    pub fn new<F>(num_threads: usize, f: F) -> Self
    where
        F: Fn(Arg) -> Res + Clone + Send + 'static,
    {
        let (res_tx, res_rx) = mpsc::channel();

        let mut threads = Vec::new();
        for _ in 0..num_threads {
            let (arg_tx, arg_rx) = mpsc::channel();
            let res_tx = res_tx.clone();
            let f = f.clone();

            let handle = thread::spawn(move || {
                // when the pool goes out of scope, the arg_tx senders will be dropped
                // so this will be an error and the thread will stop
                while let Ok((arg_index, value)) = arg_rx.recv() {
                    res_tx.send((arg_index, f(value))).map_err(|_| ())?;
                }
                Ok(())
            });
            threads.push(ThreadPoolThread {
                arg_tx,
                _handle: handle,
            });
        }

        Self { threads, res_rx }
    }

    pub fn map_args(&mut self, args: Vec<Arg>) -> impl Iterator<Item = Res> {
        let num_args = args.len();

        // send data to threads
        let mut thread_index = 0;
        for (arg_index, arg) in args.into_iter().enumerate() {
            // TODO: return error instead of unwrap?
            self.threads[thread_index]
                .arg_tx
                .send((arg_index, arg))
                .unwrap();
            thread_index = (thread_index + 1) % self.threads.len();
        }

        // get and return data from threads
        // want it to be in the same order as the input, but don't want to wait unnecessarily
        // so preallocate a vector, then insert into it at the correct location as data is received
        let mut results = (0..num_args).map(|_| None).collect::<Vec<_>>();
        for _ in 0..num_args {
            // TODO: return error instead of unwrap?
            let (i, res) = self.res_rx.recv().unwrap();
            results[i] = Some(res);
        }

        // this *should* never panic because every thread should return exactly one value before we get to this point
        results.into_iter().map(Option::unwrap)
    }
}
