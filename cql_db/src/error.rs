use std::{ io, error, fmt };

/// Error type wrapping types of errors that may be returned from cql_db
///
/// # Examples
/// The below code shows a match on the return type from [read_value](../fn.read_value.html):
/// ```
/// use cql_u64::U64;
/// use cql_db::error;
/// use std::io;
/// # const DATABASE_LOCATION: &str = "./.test_db";
/// #
/// # use std::error::Error as StdError;
/// # use std::fs::remove_file;
/// # fn main() -> Result<(), Box<dyn StdError>> {
/// # let _ = remove_file(format!("{}{}", DATABASE_LOCATION, "/db"));
/// # let _ = remove_file(format!("{}{}", DATABASE_LOCATION, "/ax"));
/// # let _ = remove_file(format!("{}{}", DATABASE_LOCATION, "/key1_2"));
/// # let _ = remove_file(format!("{}{}", DATABASE_LOCATION, "/key2_3"));
/// # cql_db::create_db::<U64>(
/// #     DATABASE_LOCATION,
/// #     &[2, 5]
/// # )?;
///
/// match cql_db::read_value::<U64>(
///     DATABASE_LOCATION,
///     &[2, 4]
/// ) {
///     Err(error) => match error {
///         error::Error::Cql(cql_error) => match cql_error {
///             error::cql::Error::DimensionTooSmallError => { },
///             error::cql::Error::IndexOutOfRangeError { dimension_index, requested, min, max } => { },
///             error::cql::Error::DimensionsOutOfRangeError { requested, min, max } => { },
///             error::cql::Error::ElementsNotLinkedError{ x_dimension, x, y_dimension, y } => { },
///         },
///         error::Error::Io(io_error) => match io_error.kind() {
///             io::ErrorKind::AlreadyExists => { },
///             io::ErrorKind::PermissionDenied => { },
///             _ => { },
///         },
///     }
///     Ok(value) => { },
/// };
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub enum Error {
    /// Any [I/O errors](https://doc.rust-lang.org/std/io/struct.Error.html) returned from a Cql function.
    Io(io::Error),
    /// Any [Cql errors](./cql/enum.Error.html) returned from a Cql function.
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

/// Cql db specific errors returned when provided with invalid parameters
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
