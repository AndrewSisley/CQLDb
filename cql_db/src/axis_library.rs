use std::fs::OpenOptions;
use std::io;

use cql_u64::U64;
use cql_model::{
    CqlType,
    CqlWritable,
    CqlReadable,
};

const AXIS_FILE_NAME: &str = "/ax";

pub struct AxisDefinition {
	pub id: u64,
	pub max: u64,
}

// The axis definitions are stored in the axis library.  The first block contains how many dimensions exist.
// The subsequent blocks contain the max size of each dimension.
pub fn create(db_location: &str, axis_definitions: &[AxisDefinition], create_new: bool) -> io::Result<()> {
    let library_axis_location = format!("{}{}", db_location, AXIS_FILE_NAME);
    OpenOptions::new()
        .write(true)
        .create(true)
        .create_new(create_new)
        .truncate(true)
        .open(&library_axis_location)
        ?
        .set_len(
            (1 + axis_definitions.len() as u64) * U64::VALUE_SIZE as u64
        )?;

    // unwrap should never fail
    U64::write_to_db(&library_axis_location, 0, axis_definitions.len() as u64).unwrap();

	for axis_definition in axis_definitions {
        // unwrap should never fail
		U64::write_to_db(&library_axis_location, axis_definition.id, axis_definition.max).unwrap();
    }

    Ok(())
}

pub fn count(db_location: &str) -> Result<u64, io::Error> {
    let library_axis_location = format!("{}{}", db_location, AXIS_FILE_NAME);

    U64::read_from_db(&library_axis_location, 0)
}

pub fn get_by_id(db_location: &str, axis_id: u64) -> Result<AxisDefinition, io::Error> {
    let library_axis_location = format!("{}{}", db_location, AXIS_FILE_NAME);

    match U64::read_from_db(&library_axis_location, axis_id) {
        Ok(max_value) => Ok(AxisDefinition { id: axis_id, max: max_value }),
        Err(e) => Err(e),
    }
}
