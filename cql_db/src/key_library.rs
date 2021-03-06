use std::io;
use std::fs::OpenOptions;

use cql_u64::U64;
use cql_model::{
    CqlType,
    CqlWritable,
    CqlReadable,
};

use crate::axis_library::AxisDefinition;
use crate::vectors::calculate_index;

const KEY_FILE_NAME: &str = "/key";

pub struct AxisPoint {
	pub axis_id: u64,
	pub position: u64
}

// The dimensions between 0..(N - 1) are mapped in the key library, allowing each 'row' in the last dimension to be added on demand
// reducing the storage space required.  Each key library contains the id of the last key added in the first block, and then acts like an 1D array
// for every point thereafter, with each entry pointing at the location of it's data in the next key library, or the start of the actual data if
// it is the penultimate dimension (N - 1).
pub fn create(db_location: &str, axis_definitions: &[AxisDefinition], create_new: bool) -> io::Result<()> {
    for index in 1..axis_definitions.len() - 1 {
        let library_key_location = format!("{}{}{}_{}", db_location, KEY_FILE_NAME, axis_definitions[index - 1].id, axis_definitions[index].id);
        OpenOptions::new()
            .write(true)
            .create(true)
            .create_new(create_new)
            .truncate(true)
            .open(library_key_location)?;
    }

    Ok(())
}

pub fn add<TStore: CqlType>(db_location: &str, x: u64, y: u64, x_axis: &AxisDefinition, y_axis: &AxisDefinition) -> Result<u64, io::Error> {
	let library_key_location = format!("{}{}{}_{}", db_location, KEY_FILE_NAME, x_axis.id, y_axis.id);
	let last_key = U64::read_from_db(&library_key_location, 0)?;

	let new_key = last_key + 1 as u64;
	let key_index = calculate_index(x, y, y_axis.max);

    U64::write_to_db(&library_key_location, 0, new_key)?;
	U64::write_to_db(&library_key_location, 1 + key_index, new_key)?;

    Ok(new_key)
}

pub fn get(db_location: &str, x: &AxisPoint, y: &AxisPoint, y_axis: &AxisDefinition) -> Result<u64, io::Error> {
	let library_key_location = format!("{}{}{}_{}", db_location, KEY_FILE_NAME, x.axis_id, y.axis_id);
	let key_location = calculate_index(x.position, y.position, y_axis.max);

    U64::read_from_db(&library_key_location, 1 + key_location)
}
