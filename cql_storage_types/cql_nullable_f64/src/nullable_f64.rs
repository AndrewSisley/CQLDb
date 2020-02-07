use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Cursor, SeekFrom, Seek};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

use cql_model::{ CqlType, CqlWritable, CqlReadable, CqlStreamReadable };

const HAS_VALUE_FLAG: u8 = 1;
const NULL_FLAG: u8 = 0;

pub struct NullableF64;

impl CqlType for NullableF64 {
    type ValueType = Option<f64>;
    const VALUE_SIZE: usize = 9;
}

impl CqlWritable for NullableF64 {
    fn write_to_db(db_location: &str, value_location: u64, input_value: Self::ValueType) {
        let mut file = OpenOptions::new().write(true).open(db_location).unwrap();

        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

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
}

impl CqlReadable for NullableF64 {
    fn read_from_db(db_location: &str, value_location: u64) -> Self::ValueType {
        let mut file = File::open(&db_location).unwrap();

        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

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
}

impl CqlStreamReadable for NullableF64 {
    fn read_to_stream(db_location: &str, stream: &mut dyn Write, value_location: u64, n_values: u64) {
        let mut file = File::open(&db_location).unwrap();

        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

        for _i in 0..n_values {
            let mut buffer = [0; 9];
            file.read(&mut buffer).unwrap();
            stream.write(&mut buffer).unwrap();
        }

        stream.flush().unwrap();
    }
}

pub fn unpack_stream<F>(stream: &mut Cursor<Vec<u8>>, n_values: usize, mut res: F) where F: FnMut(usize, Option<f64>) {
    for index in 0..n_values {
        let mut null_buffer = [0; 1];
        let mut value_buffer = [0; 8];

        match stream.read(&mut null_buffer) {
            Ok(n) => {
                if n == 0 { break; }
                else if null_buffer[0] == 0 {
                    stream.read(&mut value_buffer).unwrap();
                    res(index, None);
                }
                else {
                    let mut value_buffer = [0; 8];
                    stream.read(&mut value_buffer).unwrap();

                    let mut rdr = Cursor::new(value_buffer);
                    res(index, Some(rdr.read_f64::<LittleEndian>().unwrap()));
                }
            },
            Err(_) => panic!()
        }
    }
}
