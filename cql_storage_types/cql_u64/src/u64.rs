use std::fs::{ File, OpenOptions };
use std::io::{ Read, Write, Cursor, SeekFrom, Seek };
use byteorder::{ ReadBytesExt, WriteBytesExt, LittleEndian };

use cql_model::{ CqlType, CqlWritable, CqlReadable, CqlStreamReadable };

pub struct U64;

impl CqlType for U64 {
    type ValueType = u64;
    const VALUE_SIZE: usize = 8;
}

impl CqlWritable for U64 {
    fn write_to_db(db_location: &str, value_location: u64, value: Self::ValueType) {
        let mut file = OpenOptions::new().write(true).open(db_location).unwrap();

        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(value).unwrap();
        file.write(&wtr).unwrap();
    }
}

impl CqlReadable for U64 {
    fn read_from_db(db_location: &str, value_location: u64) -> Self::ValueType {
        let mut file = File::open(&db_location).unwrap();

        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

        let mut buffer = [0; Self::VALUE_SIZE];
        file.read(&mut buffer).unwrap();

        let mut rdr = Cursor::new(buffer);
        rdr.read_u64::<LittleEndian>().unwrap()
    }
}

impl CqlStreamReadable for U64 {
    fn read_to_stream(db_location: &str, stream: &mut dyn Write, value_location: u64, n_values: u64) {
        let mut file = File::open(&db_location).unwrap();

        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

        for _i in 0..n_values {
            let mut buffer = [0; Self::VALUE_SIZE];
            file.read(&mut buffer).unwrap();
            stream.write(&mut buffer).unwrap();
        }

        stream.flush().unwrap();
    }
}

pub fn unpack_stream<F>(stream: &mut Cursor<Vec<u8>>, n_values: usize, mut res: F) where F: FnMut(usize, u64) {
    for index in 0..n_values {
        let mut value_buffer = [0; U64::VALUE_SIZE];

        let n_bytes_read = stream.read(&mut value_buffer).unwrap();
        if n_bytes_read == 0 {
            break;
        }

        let mut rdr = Cursor::new(value_buffer);
        res(index, rdr.read_u64::<LittleEndian>().unwrap());
    }
}
