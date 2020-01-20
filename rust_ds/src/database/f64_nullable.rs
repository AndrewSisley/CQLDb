extern crate byteorder;//consider move to main

use std::fs::{File, OpenOptions}; 
use std::io::{Read, Write, Cursor, SeekFrom, Seek};
use std::net::TcpStream;
use self::byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

const VALUE_SIZE: usize = 9;
const HAS_VALUE_FLAG: u8 = 1;
const NULL_FLAG: u8 = 0;

pub fn create_db(db_location: &str, db_size: u64) {
    let file = File::create(db_location).unwrap();
    file.set_len(db_size * VALUE_SIZE as u64).unwrap();
}

pub fn write_to_db(db_location: &str, value_location: u64, input_value: Option<f64>) {
	let mut file = OpenOptions::new().write(true).open(db_location).unwrap();

	file.seek(SeekFrom::Start(value_location * VALUE_SIZE as u64)).unwrap();
    
    match input_value {
        None => { 
            file.write(&[NULL_FLAG; 1]).unwrap(); 
        }
        Some(value) => {
            file.write(&[HAS_VALUE_FLAG; 1]).unwrap();
            let mut wtr = vec![];
            wtr.write_f64::<LittleEndian>(value).unwrap();
            file.write(&wtr).unwrap();
        }
    }
}

pub fn read_from_db(db_location: &str, value_location: u64) -> Option<f64> {
	let mut file = File::open(&db_location).unwrap();

	file.seek(SeekFrom::Start(value_location * VALUE_SIZE as u64)).unwrap();

    let mut null_buffer = [0; 1];
    file.read(&mut null_buffer).unwrap();
    if null_buffer[0] == 0 {
        return None
    }

    let mut value_buffer = [0; 8];
    file.read(&mut value_buffer).unwrap();

    let mut rdr = Cursor::new(value_buffer);
    Some(rdr.read_f64::<LittleEndian>().unwrap())
}

pub fn read_to_stream(db_location: &str, stream: &mut Write, value_location: u64, n_values: u64) {println!("t0");
    let mut file = File::open(&db_location).unwrap();

    file.seek(SeekFrom::Start(value_location * VALUE_SIZE as u64)).unwrap();
    
    for _i in 0..n_values {
        let mut buffer = [0; 9];
        file.read(&mut buffer).unwrap();
        stream.write(&mut buffer);
    }

    stream.flush().unwrap();
}