use std::{ io, error, fmt };

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Cql(cql::Error),
}

impl error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref error) => error.fmt(f),
            Error::Cql(ref error) => error.fmt(f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<cql::Error> for Error {
    fn from(err: cql::Error) -> Error {
        Error::Cql(err)
    }
}

pub mod cql {
    use std::{ error, fmt };

    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    pub enum Error {
        InsufficientDimensionsError{ required: u64, requested: u64 },
        DimensionTooSmallError,
        IndexOutOfRangeError { dimension_index: usize, requested: u64, min: u64, max: u64 },
    }

    impl error::Error for Error { }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Error::InsufficientDimensionsError { required, requested } => write!(f, "Must provide at least {} dimenision(s), but only provided '{}'", required, requested),
                Error::DimensionTooSmallError => write!(f, "Dimensions must have a capacity of 1 or higher"),
                Error::IndexOutOfRangeError { dimension_index, requested, min, max } =>
                    write!(
                        f,
                        "Requested index '{}' for dimension index '{}' was out of range, value must be between {} and {}",
                        requested, dimension_index, min, max
                    ),
            }
        }
    }
}
