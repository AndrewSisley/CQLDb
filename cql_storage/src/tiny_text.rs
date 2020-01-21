use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Cursor, SeekFrom, Seek};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

use crate::internal::grow_database::grow_database;

const VALUE_SIZE: usize = (255 * 4) + 2;

pub fn grow_db(db_location: &str, size_to_grow: u64) {
    grow_database(db_location, size_to_grow, VALUE_SIZE as u64)
}

pub fn write_to_db(db_location: &str, value_location: u64, input_value: String) {
	let mut file = OpenOptions::new().write(true).open(db_location).unwrap();

    file.seek(SeekFrom::Start(value_location * VALUE_SIZE as u64)).unwrap();

    let input_length: u16 = input_value.len() as u16;
    let mut length_wtr = vec![];
    length_wtr.write_u16::<LittleEndian>(input_length).unwrap();
    file.write(&length_wtr).unwrap();

    file.write(&input_value.into_bytes()).unwrap();
}

pub fn read_from_db(db_location: &str, value_location: u64) -> String {
	let mut file = File::open(&db_location).unwrap();

	file.seek(SeekFrom::Start(value_location * VALUE_SIZE as u64)).unwrap();

    let mut size_buffer = [0; 2];
    file.read(&mut size_buffer).unwrap();

    let mut size_rdr = Cursor::new(size_buffer);
    let size = usize::from(size_rdr.read_u16::<LittleEndian>().unwrap());

    if size == 0 {
        return String::new()
    }

    let mut value_buffer = [0; VALUE_SIZE];
    file.read(&mut value_buffer).unwrap();

    let string_bytes = value_buffer[0..size].to_vec();
    String::from_utf8(string_bytes).unwrap()
}
