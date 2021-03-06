/*!
This crate implements various [CqlType](https://docs.rs/cql_model/0.2/cql_model/trait.CqlType.html) derivatives for storing `i16` values in a CQL database.

Will allocate 2 bytes per value [linked](https://docs.rs/cql_db/0.2/cql_db/fn.link_dimensions.html).

# Benchmarks
Benchmarks supplied below are fairly rudimentary (and rounded) and are there to give a rough idea of relative costs.
Full benchmark code can be found in [github](https://github.com/AndrewSisley/CQLDb/tree/master/cql_storage_types/cql_i16) and can be run with
`rustup run nightly cargo bench`.

Operation | Database dimensions | Mean time _unchecked (ns)
--- | --- | ---
Single point read | 1 | 2 520 (+/- 200)
Single point read | 4 | 15 300 (+/- 1 100)
Single point write | 1 | 2 800 (+/- 300)
Single point write | 4 | 15 350 (+/- 1 500)
Stream read 1 point | 1 | 2 500 (+/- 200)
Stream read 1 point | 4 | 15 400 (+/- 850)
Stream read 50 000 points | 1 | 27 600 000 (+/- 900 000)
Stream read 50 000 points | 4 | 27 400 000 (+/- 90 000)

# Examples
The following creates a 1D database, writes 2 values to it, and then streams them into an array.
```
# use std::io::{ Cursor, SeekFrom, Seek };
# use cql_i16::{ I16, unpack_stream };
#
# const DATABASE_LOCATION: &str = "./.test_db";
const N_VALUES_TO_READ: usize = 3;

# use std::error::Error;
# use std::fs::remove_file;
# fn main() -> Result<(), Box<dyn Error>> {
# let _ = remove_file(format!("{}{}", DATABASE_LOCATION, "/db"));
# let _ = remove_file(format!("{}{}", DATABASE_LOCATION, "/ax"));
let base_point = [1];
let value1 = 1;
let value3 = -5;

cql_db::create_db::<I16>(
    DATABASE_LOCATION,
    &[3]
)?;

cql_db::write_value::<I16>(
    DATABASE_LOCATION,
    &base_point,
    value1
)?;

cql_db::write_value::<I16>(
    DATABASE_LOCATION,
    &[base_point[0] + 2],
    value3
)?;

let mut result = [0; N_VALUES_TO_READ];
let mut stream = Cursor::new(Vec::new());

cql_db::read_to_stream::<I16>(
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
assert_eq!(result[1], 0);
assert_eq!(result[2], value3);
# Ok(())
# }
```
*/
#![doc(html_root_url = "https://docs.rs/cql_i16/0.2.0")]
use std::fs::{ File, OpenOptions };
use std::io;
use std::io::{ Read, Write, Cursor, SeekFrom, Seek };
use byteorder::{ ReadBytesExt, WriteBytesExt, LittleEndian };

use cql_model::{ CqlType, CqlWritable, CqlReadable, CqlStreamReadable };

/// Static struct for declaring that you want to work with `i16` values in a CQL database.
///
/// Stateless - used for type information only.
pub struct I16;

impl CqlType for I16 {
    type ValueType = i16;
    const VALUE_SIZE: usize = 2;
}

impl CqlWritable for I16 {
    fn write_to_db(db_location: &str, value_location: u64, value: Self::ValueType) -> io::Result<()> {
        let mut file = OpenOptions::new().write(true).open(db_location)?;

        // unwrap should be considered safe by this point, with earlier checks in the cql_db crate (if not deliberately unchecked)
        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

        let mut wtr = vec![];
        wtr.write_i16::<LittleEndian>(value)?;
        file.write_all(&wtr)
    }
}

impl CqlReadable for I16 {
    fn read_from_db(db_location: &str, value_location: u64) -> io::Result<Self::ValueType> {
        let mut file = File::open(&db_location)?;

        // unwrap should be considered safe by this point, with earlier checks in the cql_db crate (if not deliberately unchecked)
        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

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

        let mut rdr = Cursor::new(buffer);
        rdr.read_i16::<LittleEndian>()
    }
}

impl CqlStreamReadable for I16 {
    fn read_to_stream(db_location: &str, stream: &mut dyn Write, value_location: u64, n_values: u64) -> io::Result<()> {
        let mut file = File::open(&db_location)?;

        // unwrap should be considered safe by this point, with earlier checks in the cql_db crate (if not deliberately unchecked)
        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

        for _i in 0..n_values {
            let mut buffer = [0; Self::VALUE_SIZE];
            match file.read_exact(&mut buffer) {
                Err(e) => {
                    // ignore io::ErrorKind::UnexpectedEof and continue (to write '0' bytes to the writer)
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

/// Unpacks `n_values` of i16 from a stream, calling `value_handler` with each value and it's index.
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
/// cql_db::read_to_stream::<I16>(
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
pub fn unpack_stream<F>(stream: &mut Cursor<Vec<u8>>, n_values: usize, mut value_handler: F) -> io::Result<()> where F: FnMut(usize, i16) {
    for index in 0..n_values {
        let mut value_buffer = [0; I16::VALUE_SIZE];

        stream.read_exact(&mut value_buffer)?;

        let mut rdr = Cursor::new(value_buffer);
        value_handler(index, rdr.read_i16::<LittleEndian>()?);
    }

    Ok(())
}
