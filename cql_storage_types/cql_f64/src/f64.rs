#![doc(html_root_url = "https://docs.rs/cql_f64/0.1.0")]
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Cursor, SeekFrom, Seek};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

use cql_model::{ CqlType, CqlWritable, CqlReadable };

pub struct F64;

impl CqlType for F64 {
    type ValueType = f64;
    const VALUE_SIZE: usize = 8;
}

impl CqlWritable for F64 {
    fn write_to_db(db_location: &str, value_location: u64, value: f64) {
        let mut file = OpenOptions::new().write(true).open(db_location).unwrap();

        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

        let mut wtr = vec![];
        wtr.write_f64::<LittleEndian>(value).unwrap();
        file.write(&wtr).unwrap();
    }
}

impl CqlReadable for F64 {
    fn read_from_db(db_location: &str, value_location: u64) -> f64 {
        let mut file = File::open(&db_location).unwrap();

        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

        let mut buffer = [0; Self::VALUE_SIZE];
        file.read(&mut buffer).unwrap();

        let mut rdr = Cursor::new(buffer);
        rdr.read_f64::<LittleEndian>().unwrap()
    }
}
