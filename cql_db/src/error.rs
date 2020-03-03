use std::{ io, error, fmt };

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
}

impl error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref error) => error.fmt(f),
        }
    }
}
