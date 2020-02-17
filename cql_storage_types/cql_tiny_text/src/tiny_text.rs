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
Single point write | 1 | 1 | 4 070 (+/- 300)
Single point write | 255 | 1 | 4 180 (+/- 350)
Single point write | 1 | 4 | 15 100 (+/- 2 200)
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
# use std::io::{ Cursor, SeekFrom, Seek };
# use cql_tiny_text::{ TinyText, unpack_stream };
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
    value1.to_string()
);

cql_db::write_value::<TinyText>(
    DATABASE_LOCATION,
    &[base_point[0] + 2],
    value3.to_string()
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

assert_eq!(result[0], value1.to_string());
assert_eq!(result[1], String::new());
assert_eq!(result[2], value3.to_string());
```
*/
#![doc(html_root_url = "https://docs.rs/cql_tiny_text/0.1.0")]
use std::fs::{ File, OpenOptions };
use std::io::{ Read, Write, Cursor, SeekFrom, Seek };
use byteorder::{ ReadBytesExt, WriteBytesExt, LittleEndian };

use cql_model::{ CqlType, CqlWritable, CqlReadable, CqlStreamReadable };

const CONTENT_SIZE: usize = (255 * 4);
const LENGTH_SIZE: usize = 2;

/// Static struct for declaring that you want to work with `TinyText` values in a [CQL database](https://docs.rs/cql_db/0.2.0/cql_db/).
///
/// Stateless - used for type information only.
pub struct TinyText;

impl CqlType for TinyText {
    type ValueType = String;
    const VALUE_SIZE: usize = CONTENT_SIZE + LENGTH_SIZE;
}

impl CqlWritable for TinyText {
    fn write_to_db(db_location: &str, value_location: u64, input_value: Self::ValueType) {
        let mut file = OpenOptions::new().write(true).open(db_location).unwrap();

        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

        let input_length: u16 = input_value.len() as u16;
        let mut length_wtr = vec![];
        length_wtr.write_u16::<LittleEndian>(input_length).unwrap();
        file.write(&length_wtr).unwrap();

        file.write(&input_value.into_bytes()).unwrap();
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
            return String::new()
        }

        let mut value_buffer = [0; CONTENT_SIZE];
        file.read(&mut value_buffer).unwrap();

        let string_bytes = value_buffer[0..size].to_vec();
        String::from_utf8(string_bytes).unwrap()
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

/// Unpacks `n_values` of `String` from a stream, calling `res` with each value and it's index.
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
pub fn unpack_stream<F>(stream: &mut Cursor<Vec<u8>>, n_values: usize, mut res: F) where F: FnMut(usize, String) {
    let mut size_buffer = [0; LENGTH_SIZE];

    for index in 0..n_values {
        let n_bytes_read = stream.read(&mut size_buffer).unwrap();
        if n_bytes_read == 0 {
            break;
        }

        let mut size_rdr = Cursor::new(size_buffer);
        let size = usize::from(size_rdr.read_u16::<LittleEndian>().unwrap());

        if size == 0 {
            res(index, String::new());
        } else {
            let mut value_buffer = vec![0; size];
            stream.read_exact(&mut value_buffer).unwrap();
            res(index, String::from_utf8(value_buffer).unwrap());
        }
    }
}
