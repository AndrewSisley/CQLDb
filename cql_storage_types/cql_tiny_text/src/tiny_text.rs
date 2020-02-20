/*!
This crate implements various [CqlType](../cql_model/trait.CqlType.html) derivatives for storing String values of up to (and including) 255 chars in a
[CQL database](https://docs.rs/cql_db/0.2.0/cql_db/).

Will allocate 1020 bytes per value [linked](https://docs.rs/cql_db/0.2.0/cql_db/fn.link_dimensions.html).

# Benchmarks
Benchmarks supplied below are fairly rudimentary (and rounded) and are there to give a rough idea of relative costs.
Full benchmark code can be found in [github](https://github.com/AndrewSisley/CQLDb/tree/master/cql_storage_types/cql_u64) and can be run with
`rustup run nightly cargo bench`, but please be aware that they will allocate ~102MB of disk space.  The read_to_stream benchmarks also differ slightly from
other [CqlType](../cql_model/trait.CqlType.html) derivatives as they stream into a Vector, not an Array.

Operation | Chars in String | Database dimensions | Mean time (ns)
--- | --- | --- | ---
Single point read | 1 | 1 | 2 240 (+/- 185)
Single point read | 255 | 1 | 2 290 (+/- 350)
Single point read | 1 | 4 | 11 600 (+/- 2 000)
Single point read | 255 | 4 | 11 670 (+/- 4 400)
Single point write | 1 | 1 | 2 450 (+/- 500)
Single point write | 255 | 1 | 2 570 (+/- 300)
Single point write | 1 | 4 | 12 500 (+/- 2 200)
Stream read 1 point | 1 | 1 | 2 300 (+/- 500)
Stream read 1 point | 255 | 1 | 2 300 (+/- 500)
Stream read 1 point | 1 | 4 | 11 550 (+/- 2 400)
Stream read 50 000 points | 1 | 1 | 33 524 000 (+/- 540 000)
Stream read 50 000 points | 255 | 1 | 33 527 000 (+/- 650 000)
Stream read 50 000 points | 1 | 4 | 43 082 000 (+/- 867 000)
Stream read 50 000 points | 255 | 4 | 43 600 000 (+/- 1 787 000)

# Examples
The following creates a 1D database, writes 2 values to it, and then streams them into an array.
```
# use std::convert::TryFrom;
# use std::io::{ Cursor, SeekFrom, Seek };
# use cql_tiny_text::{ TinyText, unpack_stream };
#
# use std::error::Error;
# fn main() -> Result<(), Box<dyn Error>> {
#
# const DATABASE_LOCATION: &str = "./.test_db";
const N_VALUES_TO_READ: usize = 3;

let base_point = [0];
let value1 = "item one";
let value3 = "شماره ۳";

cql_db::create_db::<TinyText>(
    DATABASE_LOCATION,
    &[3]
);

cql_db::write_value::<TinyText>(
    DATABASE_LOCATION,
    &base_point,
    TinyText::try_from(value1)?
);

cql_db::write_value::<TinyText>(
    DATABASE_LOCATION,
    &[base_point[0] + 2],
    TinyText::try_from(value3)?
);

let mut result = Vec::with_capacity(N_VALUES_TO_READ);
let mut stream = Cursor::new(Vec::new());

cql_db::read_to_stream::<TinyText>(
    DATABASE_LOCATION,
    &mut stream,
    &base_point,
    N_VALUES_TO_READ as u64
);

stream.seek(SeekFrom::Start(0));
unpack_stream(&mut stream, N_VALUES_TO_READ, |_, value| {
    result.push(value)
});

assert_eq!(result[0], TinyText::try_from(value1)?);
assert_eq!(result[1], TinyText::new());
assert_eq!(result[2], TinyText::try_from(value3)?);
# Ok(())
# }
```
*/
#![doc(html_root_url = "https://docs.rs/cql_tiny_text/0.2.0")]

pub mod errors;
pub mod interop;

use std::fs::{ File, OpenOptions };
use std::io::{ Read, Write, Cursor, SeekFrom, Seek };
use byteorder::{ ReadBytesExt, WriteBytesExt, LittleEndian };

use cql_model::{ CqlType, CqlWritable, CqlReadable, CqlStreamReadable };

const CONTENT_SIZE: usize = (255 * 4);
const LENGTH_SIZE: usize = 2;

/// Tuple wrapping `String` for working with `TinyText` values in a [CQL database](https://docs.rs/cql_db/0.2.0/cql_db/).
///
/// Limited in size to `255 * 4 = 1020` bytes.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct TinyText(String);

impl CqlType for TinyText {
    type ValueType = Self;
    const VALUE_SIZE: usize = CONTENT_SIZE + LENGTH_SIZE;
}

impl TinyText {
    pub fn new() -> Self {
        TinyText(String::new())
    }
}

impl CqlWritable for TinyText {
    fn write_to_db(db_location: &str, value_location: u64, input_value: Self::ValueType) {
        let mut file = OpenOptions::new().write(true).open(db_location).unwrap();

        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

        let input_length: u16 = input_value.0.len() as u16;
        let mut buffer = vec![];
        buffer.write_u16::<LittleEndian>(input_length).unwrap();
        buffer.extend(&input_value.0.into_bytes());

        file.write(&buffer).unwrap();
    }
}

impl CqlReadable for TinyText {
    fn read_from_db(db_location: &str, value_location: u64) -> Self::ValueType {
        let mut file = File::open(&db_location).unwrap();

        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

        let mut size_buffer = [0; LENGTH_SIZE];
        file.read(&mut size_buffer).unwrap();

        let mut size_rdr = Cursor::new(size_buffer);
        let size = usize::from(size_rdr.read_u16::<LittleEndian>().unwrap());

        if size == 0 {
            return TinyText::new()
        }

        let mut value_buffer = [0; CONTENT_SIZE];
        file.read(&mut value_buffer).unwrap();

        let string_bytes = value_buffer[0..size].to_vec();
        TinyText(String::from_utf8(string_bytes).unwrap())
    }
}

impl CqlStreamReadable for TinyText {
    fn read_to_stream(db_location: &str, stream: &mut dyn Write, value_location: u64, n_values: u64) {
        let mut file = File::open(&db_location).unwrap();

        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();
        let mut value_buffer = [0; CONTENT_SIZE];

        for _i in 0..n_values {
            // must have value cleared for each value read or previous value will be quietly retained and re-written to the (out) stream
            let mut size_buffer = [0; LENGTH_SIZE];

            file.read(&mut size_buffer).unwrap();
            let mut size_rdr = Cursor::new(size_buffer);
            let size = usize::from(size_rdr.read_u16::<LittleEndian>().unwrap());

            file.read(&mut value_buffer).unwrap();

            stream.write(&mut size_buffer).unwrap();
            stream.write(&mut value_buffer[0..size]).unwrap();
        }

        stream.flush().unwrap();
    }
}

/// Unpacks `n_values` of `TinyText` from a stream, calling `res` with each value and it's index.
/// # Examples
/// ```ignore
/// cql_db::read_to_stream::<TinyText>(
///     DATABASE_LOCATION,
///     &mut stream,
///     &base_point,
///     N_VALUES_TO_READ as u64
/// );
///
/// stream.seek(SeekFrom::Start(0));
///
/// unpack_stream(&mut stream, N_VALUES_TO_READ, |idx, value| {
///     result[idx] = value
/// });
/// ```
pub fn unpack_stream<F>(stream: &mut Cursor<Vec<u8>>, n_values: usize, mut res: F) where F: FnMut(usize, TinyText) {
    let mut size_buffer = [0; LENGTH_SIZE];

    for index in 0..n_values {
        let n_bytes_read = stream.read(&mut size_buffer).unwrap();
        if n_bytes_read == 0 {
            break;
        }

        let mut size_rdr = Cursor::new(size_buffer);
        let size = usize::from(size_rdr.read_u16::<LittleEndian>().unwrap());

        if size == 0 {
            res(index, TinyText::new());
        } else {
            let mut value_buffer = vec![0; size];
            stream.read_exact(&mut value_buffer).unwrap();
            res(index, TinyText(String::from_utf8(value_buffer).unwrap()));
        }
    }
}
