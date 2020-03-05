/*!
This crate contains the core CQL Database functionality, orchestrating implementors of the [CqlType](https://docs.rs/cql_model/0.2/cql_model/trait.CqlType.html)
trait allowing the system to act as an array-based database.

The library allows the consumers to provide a path to a local directory which will be used to store array based data as defined by the user.
The number of dimensions in the array, and their maximum sizes must be stated on create of the database, however it will only allocate storage
space for elements in the final (Nth) dimension upon [linking](fn.link_dimensions_unchecked.html) of higher level dimensions.

Elements in the array can be writen to [one by one](fn.write_value_unchecked.html), and read either as [single points](fn.read_value_unchecked.html) or to a
[stream](fn.read_to_stream_unchecked.html).

# Storage space consumption

This crate will allocate file space upon linking of dimensions, as well as a small amount on create of a database, so before starting you
should be aware of the disk space requirements.

Given a database with `N` dimensions, calling [create_db_unchecked](fn.create_db_unchecked.html) will allocate `(1 + N) * 8` bytes. Thereafter,
[linking](fn.link_dimensions_unchecked.html) a set of dimensions, will then expand the maximum file sizes according to the function below:
```
# const DATABASE_LOCATION: &str = "./.test_db";
# use cql_u64::U64;
# use std::fs::OpenOptions;
#
# use std::error::Error;
# fn main() -> Result<(), Box<dyn Error>> {
#
let database_definition = [6, 7, 8, 9, 10];
let link = [2, 3, 4, 5];

# cql_db::create_db_unchecked::<U64>(
#    DATABASE_LOCATION,
#    &database_definition
# )?;
#
cql_db::link_dimensions_unchecked::<U64>(
    DATABASE_LOCATION,
    &link,
)?;

let mut key_file_size = 176; // total size of the key files in bytes
# key_file_size = OpenOptions::new().read(true).open("./.test_db/key1_2").unwrap().metadata().unwrap().len();
# key_file_size = key_file_size + OpenOptions::new().read(true).open("./.test_db/key2_3").unwrap().metadata().unwrap().len();
# key_file_size = key_file_size + OpenOptions::new().read(true).open("./.test_db/key3_4").unwrap().metadata().unwrap().len();

let n_dimensions_linked = 3; // +1 per key file
let n_elements_linked_between_second_and_third_dimension = 1; // includes this link
let n_elements_linked_between_third_and_fourth_dimension = 1; // includes this link

assert_eq!(
    (n_dimensions_linked +
        (
            (((link[0] - 1) * database_definition[1]) + link[1]) +
            (((n_elements_linked_between_second_and_third_dimension - 1) * database_definition[2]) + link[2]) +
            (((n_elements_linked_between_third_and_fourth_dimension - 1) * database_definition[3]) + link[3])
        )
    ) * 8,
    key_file_size
);
# Ok(())
# }
```
Should additional elements be linked, the key libraries will expand accordingly.

Additional space will be allocated for each penultimate dimenion `(Nn-1)` linked using the [link_dimensions_unchecked](fn.link_dimensions_unchecked.html) function, this is
equal to the maximum size of the final dimension multiplied by the [VALUE_SIZE](https://docs.rs/cql_model/0.2/cql_model/trait.CqlType.html#associatedconstant.VALUE_SIZE) of the stored struct.

# Examples

The following example creates a 4 dimensional database of unsigned 64 bit integers, links a chain of elements, writes a value, and then reads it:
```
use cql_u64::U64;

# use std::error::Error;
# fn main() -> Result<(), Box<dyn Error>> {
#
# const DATABASE_LOCATION: &str = "./.test_db";
let point = [2, 4, 3, 1];
let value = 5;

// Create a database with a maximum capacity of `[2, 5, 3, 2]`
cql_db::create_db_unchecked::<U64>(
    DATABASE_LOCATION,
    &[2, 5, 3, 2]
)?;

// Link the 2nd element of the 1st dimension with the 4th element of the 2nd dimension, and
// the 4th of the 2nd with the 3rd of the 3rd - for example:
// Turbine 2 has data for Signal 4 for Year 3
cql_db::link_dimensions_unchecked::<U64>(
    DATABASE_LOCATION,
    &[2, 4, 3], // don't link the Nth dimension, can also be expressed as `&point[0..3]`
)?;

// Write value `value` to point `point`
cql_db::write_value_unchecked::<U64>(
    DATABASE_LOCATION,
    &point,
    value
)?;

// Read the stored value from point `point`
let result = cql_db::read_value_unchecked::<U64>(
    DATABASE_LOCATION,
    &point
)?;

assert_eq!(result, value);
# Ok(())
# }
```
*/
#![doc(html_root_url = "https://docs.rs/cql_db/0.2.3")]
use std::io;
use std::io::Write;

use cql_model::{
    CqlType,
    CqlWritable,
    CqlReadable,
    CqlStreamReadable
};

mod database;
mod axis_library;
mod key_library;
mod vectors;

use axis_library::AxisDefinition;
use vectors::calculate_index;

/// Creates an CQL database in the provided directory, overwriting existing files.  Does not validate given parameters.
///
/// # Errors
///
/// Will return any [I/O errors](https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html) encountered during the execution of the function.  Function may
/// partially succeed resulting in changes to the file system, including the overwrite of any existing database files were they already present.
///
/// # Panics
///
/// Function does not actively defend against panics, and will likely do so if given invalid parameters.  Function may partially succeed resulting in changes
/// to the file system, including the overwrite of any existing database files were they already present.
///
/// # Examples
///
/// The below code creates a 2 dimensional array of [2, 3] storing unsigned 64bit integers:
/// ```
/// use cql_u64::U64;
///
/// # const DATABASE_LOCATION: &str = "./.test_db";
/// #
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// cql_db::create_db_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &[2, 3]
/// )?;
///
/// // created database will be matrix of zeros (default u64 value):
/// // [ 0, 0, 0, ]
/// // [ 0, 0, 0, ]
/// # Ok(())
/// # }
/// ```
/// More complex databases can be created by increaing the length of the input array:
/// ```
/// # use cql_u64::U64;
/// # const DATABASE_LOCATION: &str = "./.test_db";
/// #
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// // 4 dimensional array:
/// cql_db::create_db_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &[2, 3, 4, 5]
/// )?;
///
/// // 10 dimensional array:
/// cql_db::create_db_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
/// )?;
/// # Ok(())
/// # }
/// ```
/// There are no restrictions on the shape of your databases, but it is usually better to have smaller dimensions at the start:
/// ```
/// # use cql_u64::U64;
/// # const DATABASE_LOCATION: &str = "./.test_db";
/// #
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// // This is valid:
/// cql_db::create_db_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &[20, 50, 3]
/// )?;
///
/// // However this will likely be both faster to read from, and save file space:
/// cql_db::create_db_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &[3, 20, 50]
/// )?;
/// # Ok(())
/// # }
/// ```
/// But see the type(s) that you are interested in for performance benchmarks, and the [index page](./index.html) to see how to calcuate file size requirements.
pub fn create_db_unchecked<TStore: CqlType>(db_location: &str, array_size: &[u64]) -> io::Result<()> {
    let mut axis_definitions = Vec::with_capacity(array_size.len());
    for index in 0..array_size.len() {
        axis_definitions.push(AxisDefinition {
            id: index as u64 + 1,
            max: array_size[index] as u64,
        });
    }

    database::create::<TStore>(&db_location)?;
    axis_library::create(db_location, &axis_definitions)?;
    key_library::create(db_location, &axis_definitions)
}

/// Links dimension indexs together if they are not already linked.  Does not validate given parameters.
///
/// This is required before read-writing to a location, and allocates the file space required to store the Nth dimension data.
/// The last (Nth) dimension should not be linked.
///
/// # Errors
///
/// Will return any [I/O errors](https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html) encountered during the execution of the function.  Function may
/// partially succeed resulting in changes to the file system.
///
/// # Panics
///
/// Function does not actively defend against panics, and may do so if given invalid parameters.  Function may partially succeed resulting in changes
/// to the file system.
///
/// # Examples
/// ```
/// # use cql_u64::U64;
/// # const DATABASE_LOCATION: &str = "./.test_db";
/// #
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// // Create a database with a maximum capacity of `[2, 5, 3, 2]`
/// cql_db::create_db_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &[2, 5, 3, 2]
/// )?;
///
/// // Link the 2nd element of the 1st dimension with the 4th element of the 2nd dimension, and
/// // the 4th of the 2nd with the 3rd of the 3rd - for example:
/// // Turbine 2 has data for Signal 4 for Year 3
/// cql_db::link_dimensions_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &[2, 4, 3], // don't link the Nth dimension
/// )?;
/// # Ok(())
/// # }
/// ```
pub fn link_dimensions_unchecked<TStore: CqlType>(db_location: &str, location: &[u64]) -> io::Result<()> {
    let mut x_position = location[0];

    for x_axis_id in 1..location.len() {
        let y_axis_id = x_axis_id as u64 + 1;
        let y_position = location[x_axis_id];
        let y_axis_definition = axis_library::get_by_id(db_location, y_axis_id)?;

        let mut key = key_library::get(
            db_location,
            &key_library::AxisPoint { axis_id: x_axis_id as u64, position: x_position },
            &key_library::AxisPoint { axis_id: y_axis_id, position: y_position },
            &y_axis_definition
        )?;

        if key == 0 {
            key = key_library::add::<TStore>(
                db_location,
                x_position,
                y_position,
                &axis_library::get_by_id(db_location, x_axis_id as u64)?,
                &y_axis_definition
            )?;

            let last_axis_id = axis_library::count(db_location)?;
            if y_axis_id == last_axis_id - 1 {
                database::grow::<TStore>(&db_location, y_axis_definition.max)?;
            }
        };
        x_position = key;
    }

    Ok(())
}

/// Writes the given value to the given location in the database.  Does not validate given parameters.
///
/// Can result in writing to an 'alternative' location if provided with an invalid location in the final dimension, other invalid dimensions will likely
/// result in a panic.
///
/// # Errors
///
/// Will return any [I/O errors](https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html) encountered during the execution of the function.
/// If an error is returned it is not guaranteed that no bytes have been written to the requested location.
///
/// # Panics
///
/// Function does not actively defend against panics, and may do so if given invalid parameters.  If the function panics, no changes will have been made to the
/// file system and the previous value should still be present.
///
/// # Examples
/// ```
/// # use cql_u64::U64;
/// # const DATABASE_LOCATION: &str = "./.test_db";
/// #
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// cql_db::create_db_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &[2, 5, 3, 2]
/// )?;
///
/// // higher order elements must be linked before they can be writen to
/// cql_db::link_dimensions_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &[2, 4, 3],
/// )?;
///
/// // Write `5` to location `[2, 4, 3, 1]`
/// cql_db::write_value_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &[2, 4, 3, 1],
///     5
/// )?;
/// # Ok(())
/// # }
/// ```
pub fn write_value_unchecked<TStore: CqlWritable>(db_location: &str, location: &[u64], value: TStore::ValueType) -> io::Result<()> {
	let position = calculate_position(db_location, location)?;
	database::write_value::<TStore>(&db_location, position, value)
}

/// Reads the value at the given location from the database.  Does not validate given parameters.
///
/// Can result in reading from an 'alternative' location if provided with an invalid location in the final dimension, other invalid dimensions will likely
/// result in a panic.
///
/// # Errors
///
/// Will return any [I/O errors](https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html) encountered during the execution of the function.
///
/// # Panics
///
/// Function does not actively defend against panics, and may do so if given invalid parameters.
///
/// # Examples
/// ```
/// # use cql_u64::U64;
/// # const DATABASE_LOCATION: &str = "./.test_db";
/// #
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let point = [2, 4, 3, 1];
/// let value = 5;
///
/// cql_db::create_db_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &[2, 5, 3, 2]
/// )?;
///
/// // higher order elements must be linked before they can be read from
/// cql_db::link_dimensions_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &point[0..3],
/// )?;
///
/// // Read the default value from point `point`
/// let result1 = cql_db::read_value_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &point
/// )?;
///
/// assert_eq!(0, result1);
///
/// // Write `value` to location `[2, 4, 3, 1]`
/// cql_db::write_value_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &point,
///     value
/// )?;
///
/// // Read the now-populated value from point `point`
/// let result2 = cql_db::read_value_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &point
/// )?;
///
/// assert_eq!(value, result2);
/// # Ok(())
/// # }
/// ```
pub fn read_value_unchecked<TStore: CqlReadable>(db_location: &str, location: &[u64]) -> Result<TStore::ValueType, io::Error> {
	let position = calculate_position(db_location, location)?;
	database::read_value::<TStore>(&db_location, position)
}

/// Reads `n_values` from the given location onward into the given stream.  Does not validate given parameters.
///
/// Can result in reading from an 'alternative' location if provided with an invalid location in the final dimension, other invalid dimensions will likely
/// result in a panic.
///
/// # Errors
///
/// Will return any [I/O errors](https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html) encountered during the execution of the function.  If an error
/// is returned, it may be that values have already been written to the stream.
///
/// # Panics
///
/// Function does not actively defend against panics, and may do so if given invalid parameters.  If the function panics it may be that values have
/// already been written to the stream.
///
/// # Examples
/// ```
/// # use std::io::{ Cursor, SeekFrom, Seek };
/// # const DATABASE_LOCATION: &str = "./.test_db";
/// #
/// use cql_u64::{ U64, unpack_stream };
///
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let base_point = [1, 1, 1, 2];
/// const N_VALUES_TO_READ: usize = 3;
/// let value1 = 42;
/// let value2 = 16;
/// let value3 = 80;
///
/// cql_db::create_db_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &[1, 1, 1, 10]
/// )?;
///
/// cql_db::link_dimensions_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &base_point[0..3]
/// )?;
///
/// cql_db::write_value_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &base_point,
///     value1
/// )?;
///
/// cql_db::write_value_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &[1, 1, 1, base_point[3] + 1],
///     value2
/// )?;
///
/// cql_db::write_value_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &[1, 1, 1, base_point[3] + 2],
///     value3
/// )?;
///
/// let mut result = [0; N_VALUES_TO_READ];
/// let mut stream = Cursor::new(Vec::new());
///
/// cql_db::read_to_stream_unchecked::<U64>(
///     DATABASE_LOCATION,
///     &mut stream,
///     &base_point,
///     N_VALUES_TO_READ as u64
/// )?;
///
/// stream.seek(SeekFrom::Start(0));
///
/// unpack_stream(&mut stream, N_VALUES_TO_READ, |idx, value| {
///     result[idx] = value
/// })?;
///
/// assert_eq!(result[0], value1);
/// assert_eq!(result[1], value2);
/// assert_eq!(result[2], value3);
/// # Ok(())
/// # }
/// ```
pub fn read_to_stream_unchecked<TStore: CqlStreamReadable>(db_location: &str, stream: &mut dyn Write, location: &[u64], n_values: u64) -> io::Result<()> {
	let position = calculate_position(db_location, location)?;
	database::read_to_stream::<TStore>(&db_location, stream, position, n_values)
}

fn calculate_position(db_location: &str, location: &[u64]) -> io::Result<u64> {
    if location.len() == 1 {
        // minus one to handle the one-indexing
        return Ok(
            location[0] - 1
        )
    }

    let last_index = location.len() as u64 - 1;

    let mut x_position = location[0];
    for x_axis_id in 1..last_index {
        let y_axis_id = x_axis_id + 1;
        let y_position = location[x_axis_id as usize];
        let y_axis_definition = axis_library::get_by_id(db_location, y_axis_id)?;

        let key = key_library::get(
            db_location,
            &key_library::AxisPoint { axis_id: x_axis_id, position: x_position },
            &key_library::AxisPoint { axis_id: y_axis_id, position: y_position },
            &y_axis_definition
        )?;

        x_position = key;
    }

    let last_axis_definition = axis_library::get_by_id(db_location, last_index + 1)?;

    Ok(
        calculate_index(x_position, location[last_index as usize], last_axis_definition.max)
    )
}
