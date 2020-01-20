extern crate ds;

use ds::database::{ f64_nullable, u64 };
use std::io::Write;

const AXIS_FILE_NAME: &str = "/ax";
const KEY_FILE_NAME: &str = "/key";
const DB_FILE_NAME: &str = "/db";

pub fn create_db(db_location: &str, axis_definitions: &[AxisDefinition]) {
	create_axis_library(db_location, axis_definitions);
	let mut db_size: u64 = 1;

	let mut prior_definition: Option<&AxisDefinition> = None;

	let axis_definitions_depth = axis_definitions.len();
	let mut i = 1;

	for y_definition in axis_definitions {
		db_size = db_size * y_definition.max;//add check for overflow here
		if i == axis_definitions_depth {//optimize...
			break;
		}
		match prior_definition {
			Some(x_definition) => create_key_library(db_location, x_definition, y_definition),
			None => ()
		}
		prior_definition = Some(y_definition);
		i = i + 1;
	}

	let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);
	f64_nullable::create_db(&db_key_location, db_size);
}

pub fn add_key(db_location: &str, x: u64, y: u64, x_axis: &AxisDefinition, y_axis: &AxisDefinition) {//could remove fixed/starting db size and grow as these are added.
	let library_key_location = format!("{}{}{}_{}", db_location, KEY_FILE_NAME, x_axis.id, y_axis.id);
	let last_key = u64::read_from_db(&library_key_location, 0);

	let new_key = last_key + 1 as u64;
	let key_index = calc_index(x, y, y_axis.max);

	u64::write_to_db(&library_key_location, 0 as u64, new_key);
	u64::write_to_db(&library_key_location, key_index + 8 as u64, new_key);
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

//should check size here:
pub fn read_to_stream(db_location: &str, stream: &mut Write, location: &[u64], n_values: u64) {
	let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);

	let position = calculate_position(db_location, location);

	f64_nullable::read_to_stream(&db_key_location, stream, position, n_values)
}


fn create_axis_library(db_location: &str, axis_definitions: &[AxisDefinition]) {
	let library_axis_location = format!("{}{}", db_location, AXIS_FILE_NAME);
	u64::create_db(&library_axis_location, u16::max_value() as u64);

	for axis_definition in axis_definitions {
		u64::write_to_db(&library_axis_location, axis_definition.id as u64, axis_definition.max);
	}
}

fn create_key_library(db_location: &str, x_axis: &AxisDefinition, y_axis: &AxisDefinition) {//could be optimised for breeze (smaller x/y)
	let library_key_location = format!("{}{}{}_{}", db_location, KEY_FILE_NAME, x_axis.id, y_axis.id);
	u64::create_db(&library_key_location, (x_axis.max * y_axis.max) + 8 as u64);
}

fn calculate_position(db_location: &str, location: &[u64]) -> u64 {
	let mut prior_position: Option<u64> = None;

	let last_index = location.len() - 1;

	for i in 0..last_index {
		let y_axis_id = (i + 1) as u64;
		let y_position = location[i];
		let y_definition = get_axis_definition(db_location, y_axis_id);

		match prior_position {
			Some(x) => {
				let key = get_key(db_location, &AxisPoint { axis_id: i as u64, position: x },
					&AxisPoint { axis_id: y_axis_id, position: y_position }, &y_definition);
				prior_position = Some(key);
			}
			None => { 
				prior_position = Some(y_position);
			}
		}
	}

	let last_axis_definition = get_axis_definition(db_location, last_index as u64 + 1);
	calc_index(prior_position.unwrap(), location[last_index], last_axis_definition.max)
}

fn get_key(db_location: &str, x: &AxisPoint, y: &AxisPoint, y_axis: &AxisDefinition) -> u64 {
	let library_key_location = format!("{}{}{}_{}", db_location, KEY_FILE_NAME, x.axis_id, y.axis_id);
	let key_location = calc_index(x.position, y.position, y_axis.max);

	u64::read_from_db(&library_key_location, key_location + 8 as u64)
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