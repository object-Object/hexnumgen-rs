use std::{sync::Arc, thread::JoinHandle};

use parking_lot::{Condvar, Mutex, RwLock};

use crate::traits::RwLockWriteIf;

pub trait Split {
    fn num_threads(&self) -> usize;
    fn free_threads(&self) -> &Arc<RwLock<usize>>;
    fn done(&self) -> &Arc<(Mutex<bool>, Condvar)>;

    fn spawn_child(&mut self) -> JoinHandle<()>;

    fn split(&mut self) {
        // acquire a write lock if there's space for more threads to be created
        match self.free_threads().write_if(|&f| f > 0) {
            // decrement the number of free threads, then immediately release the lock so other threads can stop waiting
            // don't trigger the condition variable here because it's impossible to be done at this point
            Some(mut lock) => *lock -= 1,
            // no space, return instead of spawning a child
            None => return,
        }
        self.spawn_child();
    }

    fn merge(&self) {
        // no more work to do on this thread, so increment free_threads
        // do the increment in a block so the lock is released before acquiring the condvar mutex, to avoid deadlocks
        let free_threads = {
            let mut lock = self.free_threads().write();
            *lock += 1;
            *lock
        };
        if free_threads == self.num_threads() {
            // this is the last working thread, so tell the main thread it's safe to exit
            let mut lock = self.done().0.lock();
            *lock = true;
            self.done().1.notify_one();
        }
    }

    fn wait_until_done(&self) {
        // wait for the condition variable to trigger
        let mut lock = self.done().0.lock();
        self.done().1.wait_while(&mut lock, |done| !*done);
    }
}
