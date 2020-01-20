use cql_storage::{ f64_nullable, u64 };
use std::io::Write;
use std::mem::{ size_of };
use itertools::Itertools;

const AXIS_FILE_NAME: &str = "/ax";
const KEY_FILE_NAME: &str = "/key";
const DB_FILE_NAME: &str = "/db";

pub fn create_db(db_location: &str, axis_definitions: &[AxisDefinition]) {
    let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);
	f64_nullable::create_db(&db_key_location);

    create_axis_library(db_location, axis_definitions);

    for (x_axis, y_axis) in axis_definitions.as_ref().iter().take(axis_definitions.len() - 1).tuple_windows() {
        create_key_library(db_location, x_axis, y_axis);
    }
}

pub fn add_key(db_location: &str, x: u64, y: u64, x_axis: &AxisDefinition, y_axis: &AxisDefinition) -> u64 {
	let library_key_location = format!("{}{}{}_{}", db_location, KEY_FILE_NAME, x_axis.id, y_axis.id);
	let last_key = u64::read_from_db(&library_key_location, 0);

	let new_key = last_key + 1 as u64;
	let key_index = calc_index(x, y, y_axis.max);

    let last_axis_id = get_number_of_axis(db_location);
    if y_axis.id == last_axis_id - 1 {
        let last_axis = get_axis_definition(db_location, last_axis_id);
        let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);
        f64_nullable::grow_db(&db_key_location,last_axis.max);
    }

    u64::write_to_db(&library_key_location, 0 as u64, new_key);
	u64::write_to_db(&library_key_location, key_index + size_of::<u64>() as u64, new_key);

    new_key
}

pub fn write_value(db_location: &str, location: &[u64], value: Option<f64>) {
	let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);

	let position = calculate_position(db_location, location);

	f64_nullable::write_to_db(&db_key_location, position, value)
}

pub fn read_value(db_location: &str, location: &[u64]) -> Option<f64> {
	let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);

	let position = calculate_position(db_location, location);

	f64_nullable::read_from_db(&db_key_location, position)
}

pub fn read_to_stream(db_location: &str, stream: &mut dyn Write, location: &[u64], n_values: u64) {
	let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);

	let position = calculate_position(db_location, location);

	f64_nullable::read_to_stream(&db_key_location, stream, position, n_values)
}

fn create_axis_library(db_location: &str, axis_definitions: &[AxisDefinition]) {
	let library_axis_location = format!("{}{}", db_location, AXIS_FILE_NAME);
	u64::create_db(&library_axis_location, ((1 + axis_definitions.len()) * size_of::<u64>()) as u64);

    u64::write_to_db(&library_axis_location, 0, axis_definitions.len() as u64);

	for axis_definition in axis_definitions {
		u64::write_to_db(&library_axis_location, axis_definition.id, axis_definition.max);
	}
}

fn create_key_library(db_location: &str, x_axis: &AxisDefinition, y_axis: &AxisDefinition) {
	let library_key_location = format!("{}{}{}_{}", db_location, KEY_FILE_NAME, x_axis.id, y_axis.id);
	u64::create_db(&library_key_location, (x_axis.max * y_axis.max) + size_of::<u64>() as u64);
}

fn calculate_position(db_location: &str, location: &[u64]) -> u64 {
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

    u64::read_from_db(&library_key_location, key_location + size_of::<u64>() as u64)
}

fn get_number_of_axis(db_location: &str) -> u64 {
    let library_axis_location = format!("{}{}", db_location, AXIS_FILE_NAME);
    u64::read_from_db(&library_axis_location, 0)
}

fn get_axis_definition(db_location: &str, axis_id: u64) -> AxisDefinition {
	let library_axis_location = format!("{}{}", db_location, AXIS_FILE_NAME);
	let max_value = u64::read_from_db(&library_axis_location, axis_id);

	AxisDefinition { id: axis_id, max: max_value }
}

fn calc_index(x: u64, y: u64, y_max: u64) -> u64 {
	((x - 1) * y_max) + y//overflow check on -1!! happens if axis are not linked
}

pub struct AxisDefinition {
	pub id: u64,
	pub max: u64,
}

struct AxisPoint {
	pub axis_id: u64,
	pub position: u64
}
