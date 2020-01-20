use std::fs::{File, OpenOptions}; 
use std::io::{Read, Write, Cursor, SeekFrom, Seek};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

const VALUE_SIZE: usize = 8;

pub fn create_db(db_location: &str, db_size: u64) {
    let file = File::create(db_location).unwrap();
    file.set_len(db_size * VALUE_SIZE as u64).unwrap();
}

pub fn write_to_db(db_location: &str, value_location: u64, value: u64) {
	let mut file = OpenOptions::new().write(true).open(db_location).unwrap();

	file.seek(SeekFrom::Start(value_location * VALUE_SIZE as u64)).unwrap();

	let mut wtr = vec![];
    wtr.write_u64::<LittleEndian>(value).unwrap();
    file.write(&wtr).unwrap();
}

pub fn read_from_db(db_location: &str, value_location: u64) -> u64 {
	let mut file = File::open(&db_location).unwrap();

	file.seek(SeekFrom::Start(value_location * VALUE_SIZE as u64)).unwrap();

    let mut buffer = [0; VALUE_SIZE];
    file.read(&mut buffer).unwrap();

    let mut rdr = Cursor::new(buffer);
    rdr.read_u64::<LittleEndian>().unwrap()
}
