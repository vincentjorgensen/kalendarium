use core::fmt;

/// Returned as an error if a numeral is constructed with an invalid input
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[non_exhaustive]
pub struct OutOfRangeError;

impl fmt::Display for OutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Number out of range (must be between 1 and 9,999,999).")
    }
}
