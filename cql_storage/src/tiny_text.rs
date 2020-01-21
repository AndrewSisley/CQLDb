use std::fs::{File};
use std::io::{Read, Cursor, SeekFrom, Seek};
use byteorder::{ReadBytesExt, LittleEndian};

const VALUE_SIZE: usize = (255 * 4) + 2;

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
