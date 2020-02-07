use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Cursor, SeekFrom, Seek};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

use cql_model::{ CqlType, CqlWritable, CqlReadable };

pub struct U64;

impl CqlType for U64 {
    type ValueType = u64;
    const VALUE_SIZE: usize = 8;
}

impl CqlWritable for U64 {
    fn write_to_db(db_location: &str, value_location: u64, value: u64) {
        let mut file = OpenOptions::new().write(true).open(db_location).unwrap();

        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(value).unwrap();
        file.write(&wtr).unwrap();
    }
}

impl CqlReadable for U64 {
    fn read_from_db(db_location: &str, value_location: u64) -> u64 {
        let mut file = File::open(&db_location).unwrap();

        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

        let mut buffer = [0; Self::VALUE_SIZE];
        file.read(&mut buffer).unwrap();

        let mut rdr = Cursor::new(buffer);
        rdr.read_u64::<LittleEndian>().unwrap()
    }
}
