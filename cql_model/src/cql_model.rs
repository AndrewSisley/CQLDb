/*!
This crate contains the core models/interfaces used by CQL Database.

It does not contain any implementations.
*/
#![doc(html_root_url = "https://docs.rs/cql_model/0.2.0")]
use std::io;
use std::io::{ Write };

/// The base CQL Value Type
///
/// All read/writable types to a CQL Database should derive from this.
///
/// # Examples
/// This declares a CQL Type to read/write Strings of a maximum length of 255 bytes from the database:
/// ```
/// pub struct TinyText;
///
/// impl CqlType for TinyText {
///     type ValueType = String;
///     const VALUE_SIZE: usize = 255;
/// }
/// ```
pub trait CqlType {
    /// The type of value to read/write from the database.
    type ValueType;
    /// The (maximum) size of the value to read/write from the database.
    const VALUE_SIZE: usize;
}

/// A CQL Value Type with single point write capability.
///
/// Allows the implementing type's [Self::ValueType](trait.CqlType.html#associatedtype.ValueType) to be written to a CQL database. It should not actively
/// validate that the given parameters are valid.
///
/// # Errors
///
/// Implementations of this function should return any [I/O errors](https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html) encountered during the function.
/// If an error is returned it is not guaranteed that no bytes have been written.
///
/// [io::ErrorKind::Interrupted](https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html#variant.Interrupted) errors should also be ignored and the write
/// continued.
///
/// # Panics
///
/// Implementations are allowed to panic if the given parameters are invalid, but they do not have to.
pub trait CqlWritable: CqlType {
    /// Writes the given `input_value` to the given `value_location` in the given `db_location` (file path).
    fn write_to_db(db_location: &str, value_location: u64, input_value: Self::ValueType) -> io::Result<()>;
}

/// A CQL Value Type with single point read capability.
///
/// Allows the implementing type's [Self::ValueType](trait.CqlType.html#associatedtype.ValueType) to be read point-by-point from a CQL database.
/// It should not actively validate that the given parameters are valid.
///
/// # Errors
///
/// Implementations of this function should return any [I/O errors](https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html) encountered during the function,
/// excluding [io::ErrorKind::UnexpectedEof](https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html#variant.UnexpectedEof) errors on
/// read from the database which should result in the default value being returned.
///
/// [io::ErrorKind::Interrupted](https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html#variant.Interrupted) errors should also be ignored and the read
/// continued.
///
/// # Panics
///
/// Implementations are allowed to panic if the given parameters are invalid, but they do not have to.
pub trait CqlReadable: CqlType {
    /// Reads the value stored in the given `value_location` in the given `db_location` (file path).
    fn read_from_db(db_location: &str, value_location: u64) -> io::Result<Self::ValueType>;
}

/// A CQL Value Type with stream read capability.
///
/// Allows a range of the implementing type's [Self::ValueType](trait.CqlType.html#associatedtype.ValueType) to be read to stream from a CQL database.
/// It should not actively validate that the given parameters are valid.
///
/// # Errors
///
/// Implementations of this function should return any [I/O errors](https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html) encountered during the function,
/// excluding [io::ErrorKind::UnexpectedEof](https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html#variant.UnexpectedEof) errors on
/// read from the database which should result in default values being written to the [Write](https://doc.rust-lang.org/std/io/trait.Write.html) stream.
/// If an error is returned it is not guaranteed that no bytes have been written to the stream.
///
/// [io::ErrorKind::Interrupted](https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html#variant.Interrupted) errors should also be ignored and the read/write
/// continued.
///
/// # Panics
///
/// Implementations are allowed to panic if the given parameters are invalid, but they do not have to.
pub trait CqlStreamReadable: CqlType {
    /// Reads `n_values` from the `value_location` in the given `db_location` (file path) to the given `stream`.
    fn read_to_stream(db_location: &str, stream: &mut dyn Write, value_location: u64, n_values: u64) -> io::Result<()>;
}
