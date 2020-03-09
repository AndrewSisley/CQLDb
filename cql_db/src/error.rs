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
        DimensionTooSmallError,
        IndexOutOfRangeError { dimension_index: usize, requested: u64, min: u64, max: u64 },
        DimensionsOutOfRangeError { requested: usize, min: usize, max: usize },
        ElementsNotLinkedError { x_dimension: usize, x: u64, y_dimension: usize, y: u64 },
    }

    impl error::Error for Error { }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Error::DimensionTooSmallError => write!(f, "Dimensions must have a capacity of 1 or higher"),
                Error::IndexOutOfRangeError { dimension_index, requested, min, max } =>
                    write!(
                        f,
                        "Requested index '{}' for dimension index '{}' was out of range, value must be between {} and {}",
                        requested, dimension_index, min, max
                    ),
                Error::DimensionsOutOfRangeError { requested, min, max } =>
                    write!(f, "Requested dimension '{}' was out of range, .len() must be between {} and {}", requested, min, max),
                Error::ElementsNotLinkedError { x_dimension, x, y_dimension, y } => write!(f, "Requested dimension '{}', element '{}' was not linked to dimension '{}', element'{}",
                    x_dimension, x, y_dimension, y),
            }
        }
    }
}
