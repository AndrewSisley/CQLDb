use std::{ error::Error, fmt };

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct ValueTooLargeError;

impl Error for ValueTooLargeError { }

impl fmt::Display for ValueTooLargeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Provided value exceded maximum size")
    }
}
