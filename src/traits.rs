use num_rational::Ratio;
use parking_lot::{RwLock, RwLockWriteGuard};

pub trait UnsignedAbsRatio<UnsignedInt> {
    fn unsigned_abs(self) -> Ratio<UnsignedInt>;
}

impl UnsignedAbsRatio<u64> for Ratio<i64> {
    fn unsigned_abs(self) -> Ratio<u64> {
        Ratio::new(self.numer().unsigned_abs(), self.denom().unsigned_abs())
    }
}

pub trait AbsDiffRatio {
    fn abs_diff(self, other: Self) -> Self;
}

impl AbsDiffRatio for Ratio<u64> {
    fn abs_diff(self, other: Self) -> Self {
        if self >= other {
            self - other
        } else {
            other - self
        }
    }
}

pub trait RwLockWriteIf<T> {
    fn write_if<F>(&self, f: F) -> Option<RwLockWriteGuard<'_, T>>
    where
        F: Fn(&T) -> bool;
}

impl<T> RwLockWriteIf<T> for RwLock<T> {
    fn write_if<F>(&self, f: F) -> Option<RwLockWriteGuard<'_, T>>
    where
        F: Fn(&T) -> bool,
    {
        // first, check if the value currently matches the predicate
        // if it doesn't, no need to wait for an exclusive lock
        if f(&*self.read()) {
            // value currently matches, acquire exclusive write lock
            // then check the predicate again in case the value changed while we were waiting
            let lock = self.write();
            if f(&*lock) {
                return Some(lock);
            }
        }
        None
    }
}
