use num_traits::Zero;
use parking_lot::RwLock;
use std::{mem, sync::Arc};
use strum::IntoEnumIterator;

use crate::{hex_math::Angle, numgen::Path, threadpool::ThreadPool};

use super::BeamPathGenerator;

pub struct BeamParallelPoolPathGenerator {
    inner: BeamPathGenerator,
    pool: ThreadPool<Path, Vec<Path>>,
    pool_smallest: Arc<RwLock<Option<Path>>>,
}

impl BeamParallelPoolPathGenerator {
    pub fn new(beam_generator: BeamPathGenerator, num_threads: usize) -> Self {
        let trim_larger = beam_generator.trim_larger;
        let allow_fractions = beam_generator.allow_fractions;
        let bounds = beam_generator.bounds;
        let target = beam_generator.target;
        let pool_smallest = Arc::new(RwLock::new(None));

        let pool = {
            let pool_smallest = pool_smallest.clone();
            ThreadPool::new(num_threads, move |path: Path| {
                Angle::iter()
                    .filter_map(|angle| {
                        if let Ok(new_path) = path.with_angle(angle) {
                            if (!trim_larger || new_path.value() <= target)
                                && (allow_fractions || new_path.value().is_integer())
                                && new_path.bounds().fits_in(bounds)
                                && new_path.should_replace(&pool_smallest.read())
                            {
                                return Some(new_path);
                            }
                        }
                        None
                    })
                    .collect()
            })
        };

        Self { inner: beam_generator, pool, pool_smallest }
    }

    pub fn run(mut self) -> Option<Path> {
        if self.inner.target.is_zero() {
            return Some(self.inner.paths[0].clone());
        }
        while !self.inner.paths.is_empty() {
            self.expand();
            self.inner.trim_to_best();
            self.inner.update_smallest();
            *self.pool_smallest.write() = self.inner.smallest.clone();
        }
        self.inner.smallest
    }

    pub fn expand(&mut self) {
        let old_paths = mem::take(&mut self.inner.paths);
        self.inner.paths = self.pool.map(old_paths).into_iter().flatten().collect();
    }
}
