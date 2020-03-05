# CQLDb
Lightweight, growable, array-based storage solution, currently with the out-of-the-box storage types below (custom types also possible):
- [U64](https://crates.io/crates/cql_u64) (unsigned 64 integers)
- [F64](https://github.com/AndrewSisley/CQLDb/tree/master/cql_storage_types/cql_f64) (64-bit floating point)
- [NullableF64](https://crates.io/crates/cql_nullable_f64) (nullable 64-bit floating point)
- [TinyText](https://crates.io/crates/cql_tiny_text) (255 char utf-8 strings)

The project works by treating the file system as an N dimensional array, removing the need to scan for items in order to find them. Currently the number of dimensions must be specified on create of the database, however each dimension (bar the last) may grow on demand.

The project was originally built with an eye on storing large volumes of relational time series data, however I am looking to explore other uses in my other projects.

Database is one indexed.


## Project structure

The project is split into two core sub-projects, [cql_db](https://crates.io/crates/cql_db) and [cql_model](https://crates.io/crates/cql_model), and a sub-project per supported type within the [cql_storage_types](https://github.com/AndrewSisley/CQLDb/tree/master/cql_storage_types) folder.

The cql_db sub-project contains the core logic orchestrating the type specific logic, and the array-based logic allowing the whole thing to function as a database.  It is dependent on cql_model and the [U64](https://crates.io/crates/cql_u64) type.

The cql_model sub-project contains the interfaces consumed by cql_db and the storage types, and is referenced by all sub-projects.

The storage type specific projects contains type specific code used for read/writting specific types from a file, implementing the interfaces within the cql_model sub-project.

To use this project you'll need to import the cql_db sub-project and either each of the storage types that you with to use, or the cql_model and you own implementations of the traits within - should you wish to use other types.

Rustdocs (with examples) and crates for all published components can be found in the table below:

Repo link |Crate | Documentation | Description
--- | --- | --- | ---
[CQL Db](https://github.com/AndrewSisley/CQLDb/tree/master/cql_db) | [crates.io](https://crates.io/crates/cql_db) | [docs.rs](https://docs.rs/cql_db) | Core CQL database engine
[CQL Model](https://github.com/AndrewSisley/CQLDb/tree/master/cql_model) | [crates.io](https://crates.io/crates/cql_model) | [docs.rs](https://docs.rs/cql_model) | Core CQL database models/interfaces
[U64](https://github.com/AndrewSisley/CQLDb/tree/master/cql_storage_types/cql_u64) | [crates.io](https://crates.io/crates/cql_u64) | [docs.rs](https://docs.rs/cql_u64) | Unsigned 64-bit interger storage support
[F64](https://github.com/AndrewSisley/CQLDb/tree/master/cql_storage_types/cql_f64) | *unpublished* | *unpublished* | 64-bit floating point storage support
[NullableF64](https://github.com/AndrewSisley/CQLDb/tree/master/cql_storage_types/cql_nullable_f64) | [crates.io](https://crates.io/crates/cql_nullable_f64) | [docs.rs](https://docs.rs/cql_nullable_f64) | Nullable 64-bit floating point storage support
[TinyText](https://github.com/AndrewSisley/CQLDb/tree/master/cql_storage_types/cql_tiny_text) | [crates.io](https://crates.io/crates/cql_tiny_text) | [docs.rs](https://docs.rs/cql_tiny_text) | 255 char utf-8 string storage support


## Quick note on safety

CQL Db currently performs next to no parameter checking, and has very little deliberate error handling.  For example, calling create_db with the directory of an existing database will replace the existing one, and read_to_stream will happily wrap itself around the bounds of it's requested location if you ask it to read more points than are available.  cql_db and cql_model versions 0.2.0 and onwards start to improve this - returning errors and marking methods as '_unchecked', but it is likely that parameter checking will not be fully in play until 0.3.0.


## Breaking changes

As this project is a data-storage solution acting upon the file system, some changes may alter the expected structure of the underlying data - preventing one version from correctly utilising a database created on a different version.  Up until version 1.0.0, the minor version (middle number) will be incremented and the breaking version listed in the table below.  If you need to upgrade between one of these versions, I'd suggest reading your entire database from the earlier version and the writing it to a new database with the new target version, I will try and improve this at somepoint.  Please take care.

Crate | Breaking version | Description
--- | --- | ---
[cql_db](https://crates.io/crates/cql_db) | 0.2.0 | Changes made in the key files, and the database file itself. Commits [048e533](https://github.com/AndrewSisley/CQLDb/commit/048e533bb22602a8206a96010b86a387810ab0b2) and [7dcaf7c](https://github.com/AndrewSisley/CQLDb/commit/7dcaf7c9aa2ce7e94c7fbcf0a0e4521944790e3d)


## Getting started

To get started, pick a storage type(s) (examples use NullableF64), and add it as a dependency to your Cargo.toml, along with the core cql_db crate:

```
[dependencies]
//... (any existing dependencies you may have)
cql_db = "^0.2"
cql_nullable_f64 = "^0.2"
```

You then need to create a folder where you want the database to live, and then try out the below:

```
use std::io;
use std::io::{ Cursor, SeekFrom, Seek };
use cql_nullable_f64::{ NullableF64, unpack_stream };

const DATABASE_LOCATION: &str = "PATH_TO_YOUR_DATABASE_DIRECTORY";
const N_VALUES_TO_READ: usize = 3;

pub fn example_cql() -> io::Result<()> {
    let base_point = [1];
    let value1 = Some(-1.6);
    let value3 = Some(5.4);

    // creates a one dimensional database, with a capacity of 3
    cql_db::create_db_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &[3]
    )?;

    // writes Some(-1.6) to [0]
    cql_db::write_value_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &base_point,
        value1
    )?;

    // writes Some(5.4) to [2]
    cql_db::write_value_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &[base_point[0] + 2],
        value3
    )?;

    let mut result: [Option<f64>; N_VALUES_TO_READ] = [None; N_VALUES_TO_READ];
    let mut stream = Cursor::new(Vec::new());

    // reads 3 points from [0] into `stream`
    cql_db::read_to_stream_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &mut stream,
        &base_point,
        N_VALUES_TO_READ as u64
    )?;

    // returns to the start of the stream
    stream.seek(SeekFrom::Start(0)).unwrap();
    // unpacks the stream value by value into the result[]
    unpack_stream(&mut stream, N_VALUES_TO_READ, |idx, value| {
        result[idx] = value
    })?;

    assert_eq!(result[0], value1);
    assert_eq!(result[1], None);
    assert_eq!(result[2], value3);
}
```
More examples can be found in the [rustdocs](https://docs.rs/cql_db).

## Benchmarks

Benchmarks supplied below for the NullableF64 type and are fairly rudimentary (and rounded) and are there to give a rough idea of relative costs.
Full benchmark code can be found in [github](https://github.com/AndrewSisley/CQLDb/tree/master/cql_storage_types/cql_nullable_f64) and can be run with `rustup run nightly cargo bench`.  Benchmarks for
other types can be found in the the type's corresponding documentation.

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

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
