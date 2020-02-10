/*!
This crate implements various [CqlType](../cql_model/trait.CqlType.html) derivatives for storing `u64` values in a CQL database.

Will allocate 8 bytes per value [linked](../cql_db/fn.link_dimensions.html).

# Benchmarks
Benchmarks supplied below are fairly rudimentary (and rounded) and are there to give a rough idea of relative costs.
Full benchmark code can be found in [github](https://github.com/AndrewSisley/CQLDb/tree/master/cql_storage_types/cql_u64) and can be run with
`rustup run nightly cargo bench`.

Operation | Database dimensions | Mean time (ns)
--- | --- | ---
Single point read | 1 | 1 830 (+/- 300)
Single point read | 4 | 11 130 (+/- 1 700)
Single point write | 1 | 2 385 (+/- 600)
Single point write | 4 | 12 500 (+/- 2 200)
Stream read 1 point | 1 | 1 800 (+/- 300)
Stream read 1 point | 4 | 11 050 (+/- 1 700)
Stream read 50 000 points | 1 | 16 150 900 (+/- 200 000)
Stream read 50 000 points | 4 | 18 900 000 (+/- 160 000)

# Examples
The following creates a 1D database, writes 2 values to it, and then streams them into an array.
```
# use std::io::{ Cursor, SeekFrom, Seek };
# use cql_u64::{ U64, unpack_stream };
#
# const DATABASE_LOCATION: &str = "./.test_db";
const N_VALUES_TO_READ: usize = 3;

let base_point = [0];
let value1 = 1;
let value3 = 5;

cql_db::create_db::<U64>(
    DATABASE_LOCATION,
    &[3]
);

cql_db::write_value::<U64>(
    DATABASE_LOCATION,
    &base_point,
    value1
);

cql_db::write_value::<U64>(
    DATABASE_LOCATION,
    &[base_point[0] + 2],
    value3
);

let mut result = [0; N_VALUES_TO_READ];
let mut stream = Cursor::new(Vec::new());

cql_db::read_to_stream::<U64>(
    DATABASE_LOCATION,
    &mut stream,
    &base_point,
    N_VALUES_TO_READ as u64
);

stream.seek(SeekFrom::Start(0)).unwrap();
unpack_stream(&mut stream, N_VALUES_TO_READ, |idx, value| {
    result[idx] = value
});

assert_eq!(result[0], value1);
assert_eq!(result[1], 0);
assert_eq!(result[2], value3);
```
*/
use std::fs::{ File, OpenOptions };
use std::io::{ Read, Write, Cursor, SeekFrom, Seek };
use byteorder::{ ReadBytesExt, WriteBytesExt, LittleEndian };

use cql_model::{ CqlType, CqlWritable, CqlReadable, CqlStreamReadable };

/// Static struct for declaring that you want to work with `u64` values in a CQL database.
///
/// Stateless - used for type information only.
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
