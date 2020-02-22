use std::io::Write;
use std::fs::OpenOptions;
use cql_model::{
    CqlType,
    CqlWritable,
    CqlReadable,
    CqlStreamReadable,
};

const DB_FILE_NAME: &str = "/db";

pub fn create<TStore: CqlType>(db_location: &str) {
    let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);
    OpenOptions::new().write(true).create(true).truncate(true).open(db_key_location).unwrap();
}

pub fn grow<TStore: CqlType>(db_location: &str, size_to_grow: u64) {
    let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);
    let file = OpenOptions::new().write(true).open(db_key_location).unwrap();
    file.set_len(file.metadata().unwrap().len() + size_to_grow * TStore::VALUE_SIZE as u64).unwrap();
}

pub fn write_value<TStore: CqlWritable>(db_location: &str, value_location: u64, value: TStore::ValueType) {
	let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);
    TStore::write_to_db(&db_key_location, value_location, value).unwrap()
}

pub fn read_value<TStore: CqlReadable>(db_location: &str, value_location: u64) -> TStore::ValueType {
	let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);
	TStore::read_from_db(&db_key_location, value_location).unwrap()
}

pub fn read_to_stream<TStore: CqlStreamReadable>(db_location: &str, stream: &mut dyn Write, start_location: u64, n_values: u64) {
	let db_key_location = format!("{}{}", db_location, DB_FILE_NAME);
	TStore::read_to_stream(&db_key_location, stream, start_location, n_values).unwrap()
}
