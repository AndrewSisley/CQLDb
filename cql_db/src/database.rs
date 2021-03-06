use std::io;
use std::io::Write;
use std::fs::OpenOptions;
use cql_model::{
    CqlType,
    CqlWritable,
    CqlReadable,
    CqlStreamReadable,
};

const DB_FILE_NAME: &str = "/db";

pub fn create<TStore: CqlType>(db_location: &str, create_new: bool) -> io::Result<()> {
    let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);
    let result = OpenOptions::new()
        .write(true)
        .create(true)
        .create_new(create_new)
        .truncate(true)
        .open(db_key_location);

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn grow<TStore: CqlType>(db_location: &str, size_to_grow: u64) -> io::Result<()> {
    let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);
    let file = OpenOptions::new().write(true).open(db_key_location)?;

    file.set_len(file.metadata()?.len() + size_to_grow * TStore::VALUE_SIZE as u64)
}

pub fn write_value<TStore: CqlWritable>(db_location: &str, value_location: u64, value: TStore::ValueType) -> io::Result<()> {
	let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);
    TStore::write_to_db(&db_key_location, value_location, value)
}

pub fn read_value<TStore: CqlReadable>(db_location: &str, value_location: u64) -> Result<TStore::ValueType, io::Error> {
	let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);
	TStore::read_from_db(&db_key_location, value_location)
}

pub fn read_to_stream<TStore: CqlStreamReadable>(db_location: &str, stream: &mut dyn Write, start_location: u64, n_values: u64) -> io::Result<()> {
	let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);
	TStore::read_to_stream(&db_key_location, stream, start_location, n_values)
}
