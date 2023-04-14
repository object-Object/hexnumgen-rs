use num_rational::Ratio;

use crate::numgen::Path;

pub trait PathGenerator {
    type Opts;

    fn new(target: Ratio<i64>, trim_larger: bool, allow_fractions: bool, opts: Self::Opts) -> Self
    where
        Self: Sized;

    fn run(self) -> Option<Path>;
}
