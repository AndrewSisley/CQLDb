/*!
This crate implements various [CqlType](https://docs.rs/cql_model/0.2/cql_model/trait.CqlType.html) derivatives for storing String values of up to (and including) 255 chars in a
[CQL database](https://docs.rs/cql_db/0.2/cql_db/).

Will allocate 1020 bytes per value [linked](https://docs.rs/cql_db/0.2/cql_db/fn.link_dimensions.html).

# Benchmarks
Benchmarks supplied below are fairly rudimentary (and rounded) and are there to give a rough idea of relative costs.
Full benchmark code can be found in [github](https://github.com/AndrewSisley/CQLDb/tree/master/cql_storage_types/cql_tiny_text) and can be run with
`rustup run nightly cargo bench`, but please be aware that they will allocate ~102MB of disk space.  The read_to_stream benchmarks also differ slightly from
other [CqlType](https://docs.rs/cql_model/0.2/cql_model/trait.CqlType.html) derivatives as they stream into a Vector, not an Array.

Operation | Database dimensions | Mean time (ns)
--- | --- | ---
Single point read | 1 | 3 060 (+/- 200)
Single point read | 4 | 15 800 (+/- 1 100)
Single point write | 1 | 2 800 (+/- 300)
Single point write | 4 | 15 400 (+/- 1 000)
Stream read 1 point | 1 | 3 500 (+/- 300)
Stream read 1 point | 4 | 15 500 (+/- 1 100)
Stream read 50 000 points | 1 | 56 700 000 (+/- 800 000)
Stream read 50 000 points | 4 | 56 400 000 (+/- 150 000)

# Examples
The following creates a 1D database, writes 2 values to it, and then streams them into an array.
```
# use std::convert::TryFrom;
# use std::io::{ Cursor, SeekFrom, Seek };
# use cql_tiny_text::{ TinyText, unpack_stream };
#
# use std::error::Error;
# use std::fs::remove_file;
# fn main() -> Result<(), Box<dyn Error>> {
# let _ = remove_file(format!("{}{}", DATABASE_LOCATION, "/db"));
# let _ = remove_file(format!("{}{}", DATABASE_LOCATION, "/ax"));
#
# const DATABASE_LOCATION: &str = "./.test_db";
const N_VALUES_TO_READ: usize = 3;

let base_point = [1];
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
)?;

cql_db::write_value::<TinyText>(
    DATABASE_LOCATION,
    &[base_point[0] + 2],
    TinyText::try_from(value3)?
)?;

let mut result = Vec::with_capacity(N_VALUES_TO_READ);
let mut stream = Cursor::new(Vec::new());

cql_db::read_to_stream::<TinyText>(
    DATABASE_LOCATION,
    &mut stream,
    &base_point,
    N_VALUES_TO_READ as u64
)?;

stream.seek(SeekFrom::Start(0));
unpack_stream(&mut stream, N_VALUES_TO_READ, |_, value| {
    result.push(value)
})?;

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
use std::io;
use std::io::{ Read, Write, Cursor, SeekFrom, Seek };
use byteorder::{ ReadBytesExt, WriteBytesExt, LittleEndian };

use cql_model::{ CqlType, CqlWritable, CqlReadable, CqlStreamReadable };

const CONTENT_SIZE: usize = (255 * 4);
const LENGTH_SIZE: usize = 2;

/// Tuple wrapping `String` for working with `TinyText` values in a [CQL database](https://docs.rs/cql_db/).
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
    fn write_to_db(db_location: &str, value_location: u64, input_value: Self::ValueType) -> io::Result<()> {
        let mut file = OpenOptions::new().write(true).open(db_location)?;

        // unwrap should be considered safe by this point, with earlier checks in the cql_db crate (if not deliberately unchecked)
        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

        let input_length: u16 = input_value.0.len() as u16;
        let mut buffer = vec![];
        buffer.write_u16::<LittleEndian>(input_length)?;
        buffer.extend(&input_value.0.into_bytes());

        file.write_all(&buffer)
    }
}

impl CqlReadable for TinyText {
    fn read_from_db(db_location: &str, value_location: u64) -> io::Result<Self::ValueType> {
        let mut file = File::open(&db_location)?;

        // unwrap should be considered safe by this point, with earlier checks in the cql_db crate (if not deliberately unchecked)
        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();

        let mut size_buffer = [0; LENGTH_SIZE];
        file.read_exact(&mut size_buffer)?;

        let mut size_rdr = Cursor::new(size_buffer);
        let size = usize::from(size_rdr.read_u16::<LittleEndian>()?);

        if size == 0 {
            return Ok(TinyText::new())
        }

        let mut value_buffer = Vec::with_capacity(size);
        file.take(size as u64).read_to_end(&mut value_buffer)?;

        Ok(
            TinyText(
                // unwrap should be safe here, as we assume we are the only ones writing to the file, however low performance cost plus the fact that someone else `could`
                // write to the file discourages the use of the unsafe method that skips the checks
                String::from_utf8(value_buffer).unwrap()
            )
        )
    }
}

impl CqlStreamReadable for TinyText {
    fn read_to_stream(db_location: &str, stream: &mut dyn Write, value_location: u64, n_values: u64) -> io::Result<()> {
        let mut file = File::open(&db_location)?;

        // unwrap should be considered safe by this point, with earlier checks in the cql_db crate (if not deliberately unchecked)
        file.seek(SeekFrom::Start(value_location * Self::VALUE_SIZE as u64)).unwrap();
        let mut value_buffer = [0; CONTENT_SIZE];

        for _i in 0..n_values {
            // must have value cleared for each value read or previous value will be quietly retained and re-written to the (out) stream
            let mut size_buffer = [0; LENGTH_SIZE];

            match file.read_exact(&mut size_buffer) {
                Err(e) => {
                    // ignore io::ErrorKind::UnexpectedEof and continue
                    if e.kind() != io::ErrorKind::UnexpectedEof {
                        return Err(e)
                    }
                }
                _ => { }
            }
            let mut size_rdr = Cursor::new(size_buffer);
            let size = usize::from(size_rdr.read_u16::<LittleEndian>()?);

            match file.read_exact(&mut value_buffer) {
                Err(e) => {
                    // ignore io::ErrorKind::UnexpectedEof and continue
                    if e.kind() != io::ErrorKind::UnexpectedEof {
                        return Err(e)
                    }
                }
                _ => { }
            }

            let mut write_buffer = Vec::with_capacity(LENGTH_SIZE + size);
            write_buffer.extend(&size_buffer);
            for i in 0..size {
                write_buffer.push(value_buffer[i]);
            }

            stream.write_all(&mut write_buffer)?;
        }

        stream.flush()
    }
}

/// Unpacks `n_values` of `TinyText` from a stream, calling `value_handler` with each value and it's index.
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
/// ```ignore
/// cql_db::read_to_stream::<TinyText>(
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
pub fn unpack_stream<F>(stream: &mut Cursor<Vec<u8>>, n_values: usize, mut value_handler: F) -> io::Result<()> where F: FnMut(usize, TinyText) {
    let mut size_buffer = [0; LENGTH_SIZE];

    for index in 0..n_values {
        stream.read_exact(&mut size_buffer)?;

        let mut size_rdr = Cursor::new(size_buffer);
        let size = usize::from(size_rdr.read_u16::<LittleEndian>()?);

        if size == 0 {
            value_handler(index, TinyText::new());
        } else {
            let mut value_buffer = vec![0; size];
            stream.read_exact(&mut value_buffer)?;
            // unwrap should be safe here, as we assume we are the only ones writing to the file, however low performance cost plus the fact that someone else `could`
            // write to the file discourages the use of the unsafe method that skips the checks
            value_handler(index, TinyText(String::from_utf8(value_buffer).unwrap()));
        }
    }

    Ok(())
}
