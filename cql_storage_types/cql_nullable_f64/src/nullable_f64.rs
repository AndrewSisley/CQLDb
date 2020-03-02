/*!
This crate implements various [CqlType](https://docs.rs/cql_model/0.2/cql_model/trait.CqlType.html) derivatives for storing `Option<f64>` values in a CQL database.

Will allocate 9 bytes per value [linked](https://docs.rs/cql_db/0.2.0/cql_db/fn.link_dimensions.html).

# Benchmarks
Benchmarks supplied below are fairly rudimentary (and rounded) and are there to give a rough idea of relative costs.
Full benchmark code can be found in [github](https://github.com/AndrewSisley/CQLDb/tree/master/cql_storage_types/cql_nullable_f64) and can be run with
`rustup run nightly cargo bench`.

Operation | Database dimensions | Mean time (ns)
--- | --- | ---
Single point read | 1 | 2 200 (+/- 400)
Single point read | 4 | 11 600 (+/- 2 000)
Single point write | 1 | 2 500 (+/- 350)
Single point write | 4 | 12 500 (+/- 2 000)
Stream read 1 point | 1 | 1 860 (+/- 400)
Stream read 1 point | 4 | 11 200 (+/- 2 000)
Stream read 50 000 points | 1 | 19 800 000 (+/- 100 000)
Stream read 50 000 points | 4 | 20 150 000 (+/- 200 000)

# Examples
The following creates a 1D database, writes 2 values to it, and then streams them into an array.
```
# use std::io::{ Cursor, SeekFrom, Seek };
# use cql_nullable_f64::{ NullableF64, unpack_stream };
#
# const DATABASE_LOCATION: &str = "./.test_db";
const N_VALUES_TO_READ: usize = 3;

# use std::error::Error;
# fn main() -> Result<(), Box<dyn Error>> {
let base_point = [1];
let value1 = Some(-1.6);
let value3 = Some(5.4);

cql_db::create_db_unchecked::<NullableF64>(
    DATABASE_LOCATION,
    &[3]
)?;

cql_db::write_value_unchecked::<NullableF64>(
    DATABASE_LOCATION,
    &base_point,
    value1
)?;

cql_db::write_value_unchecked::<NullableF64>(
    DATABASE_LOCATION,
    &[base_point[0] + 2],
    value3
)?;

let mut result: [Option<f64>; N_VALUES_TO_READ] = [None; N_VALUES_TO_READ];
let mut stream = Cursor::new(Vec::new());

cql_db::read_to_stream_unchecked::<NullableF64>(
    DATABASE_LOCATION,
    &mut stream,
    &base_point,
    N_VALUES_TO_READ as u64
)?;

stream.seek(SeekFrom::Start(0)).unwrap();
unpack_stream(&mut stream, N_VALUES_TO_READ, |idx, value| {
    result[idx] = value
})?;

assert_eq!(result[0], value1);
assert_eq!(result[1], None);
assert_eq!(result[2], value3);
# Ok(())
# }
```
*/
#![doc(html_root_url = "https://docs.rs/cql_nullable_f64/0.2.0")]
use std::fs::{ File, OpenOptions };
use std::io;
use std::io::{ Read, Write, Cursor, SeekFrom, Seek };
use byteorder::{ ReadBytesExt, WriteBytesExt, LittleEndian };

use cql_model::{ CqlType, CqlWritable, CqlReadable, CqlStreamReadable };

const HAS_VALUE_FLAG: u8 = 1;
const NULL_FLAG: u8 = 0;

const CONTENT_SIZE: usize = 8;
const HAS_VALUE_SIZE: usize = 1;

/// Static struct for declaring that you want to work with `Option<f64>` values in a CQL database.
///
/// Stateless - used for type information only.
pub struct NullableF64;

impl CqlType for NullableF64 {
    type ValueType = Option<f64>;
    const VALUE_SIZE: usize = HAS_VALUE_SIZE + CONTENT_SIZE;
}

impl CqlWritable for NullableF64 {
    fn write_to_db(db_location: &str, value_location: u64, input_value: Self::ValueType) -> io::Result<()> {
        let mut file = OpenOptions::new().write(true).open(db_location)?;

        // unwrap should be considered safe by this point, with earlier checks in the cql_db crate (if not deliberately unchecked)
        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

        match input_value {
            None => {
                file.write_all(&[NULL_FLAG; HAS_VALUE_SIZE])
            }
            Some(value) => {
                let mut buffer = vec![HAS_VALUE_FLAG];
                buffer.write_f64::<LittleEndian>(value)?;
                file.write_all(&buffer)
            }
        }
    }
}

impl CqlReadable for NullableF64 {
    fn read_from_db(db_location: &str, value_location: u64) -> io::Result<Self::ValueType> {
        let mut file = File::open(&db_location)?;

        // unwrap should be considered safe by this point, with earlier checks in the cql_db crate (if not deliberately unchecked)
        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

        let mut null_buffer = [0; HAS_VALUE_SIZE];
        match file.read_exact(&mut null_buffer) {
            Err(e) => {
                // ignore io::ErrorKind::UnexpectedEof and continue
                if e.kind() != io::ErrorKind::UnexpectedEof {
                    return Err(e)
                }
            }
            _ => { }
        }

        if null_buffer[0] == NULL_FLAG {
            return Ok(None)
        }

        let mut value_buffer = [0; CONTENT_SIZE];
        file.read_exact(&mut value_buffer)?;

        let mut rdr = Cursor::new(value_buffer);
        Ok(Some(rdr.read_f64::<LittleEndian>()?))
    }
}

impl CqlStreamReadable for NullableF64 {
    fn read_to_stream(db_location: &str, stream: &mut dyn Write, value_location: u64, n_values: u64) -> io::Result<()> {
        let mut file = File::open(&db_location)?;

        // unwrap should be considered safe by this point, with earlier checks in the cql_db crate (if not deliberately unchecked)
        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

        for _i in 0..n_values {
            let mut buffer = [0; Self::VALUE_SIZE];
            match file.read_exact(&mut buffer) {
                Err(e) => {
                    // ignore io::ErrorKind::UnexpectedEof and continue
                    if e.kind() != io::ErrorKind::UnexpectedEof {
                        return Err(e)
                    }
                }
                _ => { }
            }
            stream.write_all(&mut buffer)?;
        }

        stream.flush()
    }
}

/// Unpacks `n_values` of Option<f64> from a stream, calling `value_handler` with each value and it's index.
///
/// # Errors
///
/// Will return any [I/O errors](https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html) encountered during the execution of the function.  If an error
/// is returned, it may be that values have already been fed into the `value_handler`.
///
/// # Panics
///
/// Function does not actively defend against panics, and may do so if given invalid parameters.  If the function panics it may be that values have
/// already been fed into the `value_handler`.
///
/// # Examples
///
/// ```ignore
/// cql_db::read_to_stream_unchecked::<NullableF64>(
///     DATABASE_LOCATION,
///     &mut stream,
///     &base_point,
///     N_VALUES_TO_READ as u64
/// )?;
///
/// stream.seek(SeekFrom::Start(0));
///
/// unpack_stream(&mut stream, N_VALUES_TO_READ, |idx, value| {
///     result[idx] = value
/// })?;
/// ```
pub fn unpack_stream<F>(stream: &mut Cursor<Vec<u8>>, n_values: usize, mut value_handler: F) -> io::Result<()> where F: FnMut(usize, Option<f64>) {
    for index in 0..n_values {
        let mut buffer = [0; NullableF64::VALUE_SIZE];
        stream.read_exact(&mut buffer)?;

        if buffer[0] == NULL_FLAG {
            value_handler(index, None);
        } else {
            let mut rdr = Cursor::new(&buffer[1..NullableF64::VALUE_SIZE]);
            value_handler(index, Some(rdr.read_f64::<LittleEndian>()?));
        }
    }

    Ok(())
}
