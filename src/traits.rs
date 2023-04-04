use num_rational::Ratio;

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
