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

pub fn create_db<TStore: CqlType>(db_location: &str, array_size: &[u64]) {
    let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);
    TStore::create_db(&db_key_location);

    let mut axis_definitions = Vec::with_capacity(array_size.len());
    for index in 0..array_size.len() {
        axis_definitions.push(AxisDefinition {
            id: index as u64 + 1,
            max: array_size[index] as u64,
        });
    }

    create_axis_library(db_location, &axis_definitions);

    for index in 1..axis_definitions.len() - 1 {
        create_key_library(db_location, &axis_definitions[index - 1], &axis_definitions[index]);
    }
}

// Links dimension indexs together if they are not already linked.
// This is required before read-writing to a location, and allocates the file space required to store the Nth dimension data.
// The last (Nth) dimension should not be linked.
pub fn link_dimensions<TStore: CqlType>(db_location: &str, location: &[u64]) {
    let mut x_position = location[0];

    for x_axis_id in 1..(location.len()) {
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

pub fn write_value<TStore: CqlWritable>(db_location: &str, location: &[u64], value: TStore::ValueType) {
	let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);

	let position = calculate_position(db_location, location);

	TStore::write_to_db(&db_key_location, position, value)
}

pub fn read_value<TStore: CqlReadable>(db_location: &str, location: &[u64]) -> TStore::ValueType {
	let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);

	let position = calculate_position(db_location, location);

	TStore::read_from_db(&db_key_location, position)
}

pub fn read_to_stream<TStore: CqlStreamReadable>(db_location: &str, stream: &mut dyn Write, location: &[u64], n_values: u64) {
	let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);

	let position = calculate_position(db_location, location);

	TStore::read_to_stream(&db_key_location, stream, position, n_values)
}

// The axis definitions are stored in the axis library.  The first block contains how many dimensions exist.
// The subsequent blocks contain the max size of each dimension.
fn create_axis_library(db_location: &str, axis_definitions: &[AxisDefinition]) {
	let library_axis_location = format!("{}{}", db_location, AXIS_FILE_NAME);
    U64::create_db(&library_axis_location);
    grow_database(&library_axis_location, 1 + axis_definitions.len() as u64, U64::VALUE_SIZE);

    U64::write_to_db(&library_axis_location, 0, axis_definitions.len() as u64);

	for axis_definition in axis_definitions {
		U64::write_to_db(&library_axis_location, axis_definition.id, axis_definition.max);
	}
}

// The dimensions between 0..(N - 1) are mapped in the key library, allowing each 'row' in the last dimension to be added on demand
// reducing the storage space required.  Each key library contains the id of the last key added in the first block, and then acts like an 1D array
// for every point thereafter, with each entry pointing at the location of it's data in the next key library, or the start of the actual data if
// it is the penultimate dimension (N - 1).
fn create_key_library(db_location: &str, x_axis: &AxisDefinition, y_axis: &AxisDefinition) {
	let library_key_location = format!("{}{}{}_{}", db_location, KEY_FILE_NAME, x_axis.id, y_axis.id);
	U64::create_db(&library_key_location);
    grow_database(&library_key_location, 1 + (x_axis.max * y_axis.max), U64::VALUE_SIZE);
}

fn grow_database(db_location: &str, size_to_grow: u64, value_size: usize) {
    // FIXME: Need to make sure that this operation is atomic.
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
	U64::write_to_db(&library_key_location, 1 + key_index, new_key);

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

    U64::read_from_db(&library_key_location, 1 + key_location)
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
