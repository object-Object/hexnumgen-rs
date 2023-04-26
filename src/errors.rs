use png::EncodingError;
use thiserror::Error;

use crate::hex_math::{Angle, Segment};

#[derive(Error, Debug)]
pub enum HexError {
    #[error("invalid character `{0}`")]
    InvalidChar(char),
    #[error("invalid angle `{0:?}`")]
    InvalidAngle(Angle),
    #[error("invalid angle `{0:?}` for number `{1}`")]
    InvalidAngleForNumber(Angle, u32),
    #[error("segment `{0:?}` already exists in path")]
    SegmentAlreadyExists(Segment),
    #[error("value is valid but would be outside of PathLimits")]
    OutOfLimits,
    #[error("PNG encoding error")]
    EncodingError(#[from] EncodingError),
    #[error("invalid pattern")]
    InvalidPattern,
    #[error("invalid direction `{0}`")]
    InvalidDirection(String),
}

pub type HexResult<T> = Result<T, HexError>;
