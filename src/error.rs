/// The `Error` type for the `mayheap` crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Attempted to grow a collection beyond its capacity.
    ///
    /// This error can only occur when `heapless` feature is enabled.
    BufferOverflow,
    /// Invalid UTF-8 sequence.
    Utf8Error(core::str::Utf8Error),
}

/// The Result type for the zlink crate.
pub type Result<T> = core::result::Result<T, Error>;

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Error::BufferOverflow => None,
            Error::Utf8Error(err) => Some(err),
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::BufferOverflow => {
                write!(f, "Attempted to grow a collection beyond its capacity")
            }
            Error::Utf8Error(err) => {
                write!(f, "Invalid UTF-8 sequence: {err}")
            }
        }
    }
}

impl From<core::str::Utf8Error> for Error {
    fn from(err: core::str::Utf8Error) -> Self {
        Error::Utf8Error(err)
    }
}
