/*!
This crate contains the core models/interfaces used by CQL Database.

It does not contain any implementations.
*/
#![doc(html_root_url = "https://docs.rs/cql_model/0.1.0")]
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
/// Allows the implementing type's [Self::ValueType](trait.CqlType.html#associatedtype.ValueType) to be written to a CQL database.
pub trait CqlWritable: CqlType {
    /// Writes the given `input_value` to the given `value_location` in the given `db_location` (file path).
    fn write_to_db(db_location: &str, value_location: u64, input_value: Self::ValueType);
}

/// A CQL Value Type with single point read capability.
///
/// Allows the implementing type's [Self::ValueType](trait.CqlType.html#associatedtype.ValueType) to be read point-by-point from a CQL database.
pub trait CqlReadable: CqlType {
    /// Reads the value stored in the given `value_location` in the given `db_location` (file path).
    fn read_from_db(db_location: &str, value_location: u64) -> Self::ValueType;
}

/// A CQL Value Type with stream read capability.
///
/// Allows a range of the implementing type's [Self::ValueType](trait.CqlType.html#associatedtype.ValueType) to be read to stream from a CQL database.
pub trait CqlStreamReadable: CqlType {
    /// Reads `n_values` from the `value_location` in the given `db_location` (file path) to the given `stream`.
    fn read_to_stream(db_location: &str, stream: &mut dyn Write, value_location: u64, n_values: u64);
}
