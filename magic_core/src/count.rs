/// The number of things to select.
///
/// Maximum is exclusive.
#[derive(Clone, Debug)]
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
