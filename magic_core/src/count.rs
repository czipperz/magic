/// The number of things to select.
///
/// Maximum is inclusive.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Count {
    pub minimum: usize,
    pub maximum: Option<usize>,
}

impl From<usize> for Count {
    fn from(x: usize) -> Count {
        Count {
            minimum: x,
            maximum: Some(x),
        }
    }
}

use std::ops::{RangeFrom, RangeInclusive};
impl From<RangeInclusive<usize>> for Count {
    fn from(range: RangeInclusive<usize>) -> Count {
        let (start, end) = range.into_inner();
        assert!(end >= start);
        Count {
            minimum: start,
            maximum: Some(end),
        }
    }
}

impl From<RangeFrom<usize>> for Count {
    fn from(range: RangeFrom<usize>) -> Count {
        Count {
            minimum: range.start,
            maximum: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_from_usize() {
        assert_eq!(
            Count::from(2),
            Count {
                minimum: 2,
                maximum: Some(2),
            }
        );
    }

    #[test]
    fn test_range_inclusive() {
        assert_eq!(
            Count::from(1..=1),
            Count {
                minimum: 1,
                maximum: Some(1),
            }
        );
    }

    #[test]
    #[should_panic]
    fn test_range_inclusive_panics_if_maximum_less_than_minimum() {
        Count::from(1..=0);
    }

    #[test]
    fn test_range_infinite() {
        assert_eq!(
            Count::from(1..),
            Count {
                minimum: 1,
                maximum: None,
            }
        );
    }
}
