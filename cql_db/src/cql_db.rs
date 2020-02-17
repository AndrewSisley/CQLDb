/*!
This crate contains the core CQL Database functionality, orchestrating implementors of the [CqlType](../cql_model/trait.CqlType.html)
trait allowing the system to act as an array-based database.

The library allows the consumers to provide a path to a local directory which will be used to store array based data as defined by the user.
The number of dimensions in the array, and their maximum sizes must be stated on create of the database, however it will only allocate storage
space for elements in the final (Nth) dimension upon [linking](fn.link_dimensions.html) of higher level dimensions.

Elements in the array can be writen to [one by one](fn.write_value.html), and read either as [single points](fn.read_value.html) or to a
[stream](fn.read_to_stream.html).

# Storage space consumption

This crate will allocate file space upon linking of dimensions, as well as a small amount on create of a database, so before starting you
should be aware of the disk space requirements.

Given a database with `N` dimensions, calling [create_db](fn.create_db.html) will allocate `(1 + N) * 8` bytes. Thereafter,
[linking](fn.link_dimensions.html) a set of dimensions, will then expand the maximum file sizes according to the function below:
```
# const DATABASE_LOCATION: &str = "./.test_db";
# use cql_u64::U64;
# use std::fs::OpenOptions;
#
let database_definition = [6, 7, 8, 9, 10];
let link = [2, 3, 4, 5];

# cql_db::create_db::<U64>(
#    DATABASE_LOCATION,
#    &database_definition
# );
#
cql_db::link_dimensions::<U64>(
    DATABASE_LOCATION,
    &link,
);

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
```
Should additional elements be linked, the key libraries will expand accordingly.

Additional space will be allocated for each penultimate dimenion `(Nn-1)` linked using the [link_dimensions](fn.link_dimensions.html) function, this is
equal to the maximum size of the final dimension multiplied by the [VALUE_SIZE](../cql_model/trait.CqlType.html#associatedconstant.VALUE_SIZE) of the stored struct.

# Examples

The following example creates a 4 dimensional database of unsigned 64 bit integers, links a chain of elements, writes a value, and then reads it:
```
use cql_u64::U64;

# const DATABASE_LOCATION: &str = "./.test_db";
let point = [2, 4, 3, 1];
let value = 5;

// Create a database with a maximum capacity of `[2, 5, 3, 2]`
cql_db::create_db::<U64>(
    DATABASE_LOCATION,
    &[2, 5, 3, 2]
);

// Link the 2nd element of the 1st dimension with the 4th element of the 2nd dimension, and
// the 4th of the 2nd with the 3rd of the 3rd - for example:
// Turbine 2 has data for Signal 4 for Year 3
cql_db::link_dimensions::<U64>(
    DATABASE_LOCATION,
    &[2, 4, 3], // don't link the Nth dimension, can also be expressed as `&point[0..3]`
);

// Write value `value` to point `point`
cql_db::write_value::<U64>(
    DATABASE_LOCATION,
    &point,
    value
);

// Read the stored value from point `point`
let result = cql_db::read_value::<U64>(
    DATABASE_LOCATION,
    &point
);

assert_eq!(result, value);
```
*/
#![doc(html_root_url = "https://docs.rs/cql_db/0.2.0")]
use std::io::Write;
use std::fs::OpenOptions;

use cql_u64::U64;
use cql_model::{
    CqlType,
    CqlWritable,
    CqlReadable,
    CqlStreamReadable
};

const AXIS_FILE_NAME: &str = "/ax";
const KEY_FILE_NAME: &str = "/key";
const DB_FILE_NAME: &str = "/db";

/// Creates an CQL database in the provided directory, overwriting existing files.
pub fn create_db<TStore: CqlType>(db_location: &str, array_size: &[u64]) {
    let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);
    create_file(&db_key_location, 0);

    let mut axis_definitions = Vec::with_capacity(array_size.len());
    for index in 0..array_size.len() {
        axis_definitions.push(AxisDefinition {
            id: index as u64 + 1,
            max: array_size[index] as u64,
        });
    }

    create_axis_library(db_location, &axis_definitions);

    for index in 1..axis_definitions.len() - 1 {
        create_key_library(db_location, axis_definitions[index - 1].id, axis_definitions[index].id);
    }
}

/// Links dimension indexs together if they are not already linked.
///
/// This is required before read-writing to a location, and allocates the file space required to store the Nth dimension data.
/// The last (Nth) dimension should not be linked.
///
/// # Examples
/// ```
/// # use cql_u64::U64;
/// # const DATABASE_LOCATION: &str = "./.test_db";
/// #
/// // Create a database with a maximum capacity of `[2, 5, 3, 2]`
/// cql_db::create_db::<U64>(
///     DATABASE_LOCATION,
///     &[2, 5, 3, 2]
/// );
///
/// // Link the 2nd element of the 1st dimension with the 4th element of the 2nd dimension, and
/// // the 4th of the 2nd with the 3rd of the 3rd - for example:
/// // Turbine 2 has data for Signal 4 for Year 3
/// cql_db::link_dimensions::<U64>(
///     DATABASE_LOCATION,
///     &[2, 4, 3], // don't link the Nth dimension
/// );
/// ```
pub fn link_dimensions<TStore: CqlType>(db_location: &str, location: &[u64]) {
    let mut x_position = location[0];

    for x_axis_id in 1..location.len() {
        let y_axis_id = x_axis_id as u64 + 1;
        let y_position = location[x_axis_id];
        let y_axis_definition = get_axis_definition(db_location, y_axis_id);

        let mut key = get_key(
            db_location,
            &AxisPoint { axis_id: x_axis_id as u64, position: x_position },
            &AxisPoint { axis_id: y_axis_id, position: y_position },
            &y_axis_definition
        );

        if key == 0 {
            key = add_key::<TStore>(
                db_location,
                x_position,
                y_position,
                &get_axis_definition(db_location, x_axis_id as u64),
                &y_axis_definition
            );
        };
        x_position = key;
    }
}

/// Writes the given value to the given location in the database.
///
/// # Examples
/// ```
/// # use cql_u64::U64;
/// # const DATABASE_LOCATION: &str = "./.test_db";
/// #
/// cql_db::create_db::<U64>(
///     DATABASE_LOCATION,
///     &[2, 5, 3, 2]
/// );
///
/// // higher order elements must be linked before they can be writen to
/// cql_db::link_dimensions::<U64>(
///     DATABASE_LOCATION,
///     &[2, 4, 3],
/// );
///
/// // Write `5` to location `[2, 4, 3, 1]`
/// cql_db::write_value::<U64>(
///     DATABASE_LOCATION,
///     &[2, 4, 3, 1],
///     5
/// );
/// ```
pub fn write_value<TStore: CqlWritable>(db_location: &str, location: &[u64], value: TStore::ValueType) {
	let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);

	let position = calculate_position(db_location, location);

	TStore::write_to_db(&db_key_location, position, value)
}

/// Reads the value at the given location from the database.
///
/// # Examples
/// ```
/// # use cql_u64::U64;
/// # const DATABASE_LOCATION: &str = "./.test_db";
/// #
/// let point = [2, 4, 3, 1];
/// let value = 5;
///
/// cql_db::create_db::<U64>(
///     DATABASE_LOCATION,
///     &[2, 5, 3, 2]
/// );
///
/// // higher order elements must be linked before they can be read from
/// cql_db::link_dimensions::<U64>(
///     DATABASE_LOCATION,
///     &point[0..3],
/// );
///
/// // Read the default value from point `point`
/// let result1 = cql_db::read_value::<U64>(
///     DATABASE_LOCATION,
///     &point
/// );
///
/// assert_eq!(0, result1);
///
/// // Write `value` to location `[2, 4, 3, 1]`
/// cql_db::write_value::<U64>(
///     DATABASE_LOCATION,
///     &point,
///     value
/// );
///
/// // Read the now-populated value from point `point`
/// let result2 = cql_db::read_value::<U64>(
///     DATABASE_LOCATION,
///     &point
/// );
///
/// assert_eq!(value, result2);
/// ```
pub fn read_value<TStore: CqlReadable>(db_location: &str, location: &[u64]) -> TStore::ValueType {
	let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);

	let position = calculate_position(db_location, location);

	TStore::read_from_db(&db_key_location, position)
}

/// Reads `n_values` from the given location onward into the given stream.
///
/// Does not offer any protection against reading more values than exist in the selected location.
///
/// # Examples
/// ```
/// # use std::io::{ Cursor, SeekFrom, Seek };
/// # const DATABASE_LOCATION: &str = "./.test_db";
/// #
/// use cql_u64::{ U64, unpack_stream };
///
/// let base_point = [1, 1, 1, 2];
/// const N_VALUES_TO_READ: usize = 3;
/// let value1 = 42;
/// let value2 = 16;
/// let value3 = 80;
///
/// cql_db::create_db::<U64>(
///     DATABASE_LOCATION,
///     &[1, 1, 1, 10]
/// );
///
/// cql_db::link_dimensions::<U64>(
///     DATABASE_LOCATION,
///     &base_point[0..3]
/// );
///
/// cql_db::write_value::<U64>(
///     DATABASE_LOCATION,
///     &base_point,
///     value1
/// );
///
/// cql_db::write_value::<U64>(
///     DATABASE_LOCATION,
///     &[1, 1, 1, base_point[3] + 1],
///     value2
/// );
///
/// cql_db::write_value::<U64>(
///     DATABASE_LOCATION,
///     &[1, 1, 1, base_point[3] + 2],
///     value3
/// );
///
/// let mut result = [0; N_VALUES_TO_READ];
/// let mut stream = Cursor::new(Vec::new());
///
/// cql_db::read_to_stream::<U64>(
///     DATABASE_LOCATION,
///     &mut stream,
///     &base_point,
///     N_VALUES_TO_READ as u64
/// );
///
/// stream.seek(SeekFrom::Start(0));
///
/// unpack_stream(&mut stream, N_VALUES_TO_READ, |idx, value| {
///     result[idx] = value
/// });
///
/// assert_eq!(result[0], value1);
/// assert_eq!(result[1], value2);
/// assert_eq!(result[2], value3);
/// ```
pub fn read_to_stream<TStore: CqlStreamReadable>(db_location: &str, stream: &mut dyn Write, location: &[u64], n_values: u64) {
	let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);

	let position = calculate_position(db_location, location);

	TStore::read_to_stream(&db_key_location, stream, position, n_values)
}

fn create_file(db_location: &str, size: u64) {
    let file = OpenOptions::new().write(true).create(true).truncate(true).open(db_location).unwrap();
    file.set_len(size).unwrap();
}

// The axis definitions are stored in the axis library.  The first block contains how many dimensions exist.
// The subsequent blocks contain the max size of each dimension.
fn create_axis_library(db_location: &str, axis_definitions: &[AxisDefinition]) {
	let library_axis_location = format!("{}{}", db_location, AXIS_FILE_NAME);
    create_file(&library_axis_location, (1 + axis_definitions.len() as u64) * U64::VALUE_SIZE as u64);

    U64::write_to_db(&library_axis_location, 0, axis_definitions.len() as u64);

	for axis_definition in axis_definitions {
		U64::write_to_db(&library_axis_location, axis_definition.id, axis_definition.max);
	}
}

// The dimensions between 0..(N - 1) are mapped in the key library, allowing each 'row' in the last dimension to be added on demand
// reducing the storage space required.  Each key library contains the id of the last key added in the first block, and then acts like an 1D array
// for every point thereafter, with each entry pointing at the location of it's data in the next key library, or the start of the actual data if
// it is the penultimate dimension (N - 1).
fn create_key_library(db_location: &str, x_axis_id: u64, y_axis_id: u64) {
	let library_key_location = format!("{}{}{}_{}", db_location, KEY_FILE_NAME, x_axis_id, y_axis_id);
	create_file(&library_key_location, 0);
}

fn grow_database(db_location: &str, size_to_grow: u64, value_size: usize) {
    let file = OpenOptions::new().write(true).open(db_location).unwrap();
    file.set_len(file.metadata().unwrap().len() + size_to_grow * value_size as u64).unwrap();
}

fn add_key<TStore: CqlType>(db_location: &str, x: u64, y: u64, x_axis: &AxisDefinition, y_axis: &AxisDefinition) -> u64 {
	let library_key_location = format!("{}{}{}_{}", db_location, KEY_FILE_NAME, x_axis.id, y_axis.id);
	let last_key = U64::read_from_db(&library_key_location, 0);

	let new_key = last_key + 1 as u64;
	let key_index = calc_index(x, y, y_axis.max);

    let last_axis_id = get_number_of_axis(db_location);
    if y_axis.id == last_axis_id - 1 {
        let last_axis = get_axis_definition(db_location, last_axis_id);
        let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);
        grow_database(&db_key_location, last_axis.max, TStore::VALUE_SIZE);
    }

    U64::write_to_db(&library_key_location, 0, new_key);
	U64::write_to_db(&library_key_location, key_index, new_key);

    new_key
}

fn calculate_position(db_location: &str, location: &[u64]) -> u64 {
    if location.len() == 1 {
        return location[0]
    }

    let last_index = location.len() as u64 - 1;

    let mut x_position = location[0];
    for x_axis_id in 1..last_index {
        let y_axis_id = x_axis_id + 1;
        let y_position = location[x_axis_id as usize];
        let y_axis_definition = get_axis_definition(db_location, y_axis_id);

        let key = get_key(
            db_location,
            &AxisPoint { axis_id: x_axis_id, position: x_position },
            &AxisPoint { axis_id: y_axis_id, position: y_position },
            &y_axis_definition
        );

        x_position = key;
    }

    let last_axis_definition = get_axis_definition(db_location, last_index + 1);
    calc_index(x_position, location[last_index as usize], last_axis_definition.max)
}

fn get_key(db_location: &str, x: &AxisPoint, y: &AxisPoint, y_axis: &AxisDefinition) -> u64 {
	let library_key_location = format!("{}{}{}_{}", db_location, KEY_FILE_NAME, x.axis_id, y.axis_id);
	let key_location = calc_index(x.position, y.position, y_axis.max);

    U64::read_from_db(&library_key_location, key_location)
}

fn get_number_of_axis(db_location: &str) -> u64 {
    let library_axis_location = format!("{}{}", db_location, AXIS_FILE_NAME);
    U64::read_from_db(&library_axis_location, 0)
}

fn get_axis_definition(db_location: &str, axis_id: u64) -> AxisDefinition {
	let library_axis_location = format!("{}{}", db_location, AXIS_FILE_NAME);
	let max_value = U64::read_from_db(&library_axis_location, axis_id);

	AxisDefinition { id: axis_id, max: max_value }
}

fn calc_index(x: u64, y: u64, y_max: u64) -> u64 {
	((x - 1) * y_max) + y//overflow check on -1!! happens if axis are not linked
}

struct AxisDefinition {
	pub id: u64,
	pub max: u64,
}

struct AxisPoint {
	pub axis_id: u64,
	pub position: u64
}
