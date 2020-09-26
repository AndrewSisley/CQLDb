# CQLDb
Lightweight, growable, array-based storage solution, currently with the out-of-the-box storage types below (custom types also possible):
- [I16](https://crates.io/crates/cql_i16) (signed 16-bit integers)
- [U64](https://crates.io/crates/cql_u64) (unsigned 64-bit integers)
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
[I16](https://github.com/AndrewSisley/CQLDb/tree/master/cql_storage_types/cql_i16) | [crates.io](https://crates.io/crates/cql_i16) | [docs.rs](https://docs.rs/cql_i16) | Signed 16-bit integer storage support
[U64](https://github.com/AndrewSisley/CQLDb/tree/master/cql_storage_types/cql_u64) | [crates.io](https://crates.io/crates/cql_u64) | [docs.rs](https://docs.rs/cql_u64) | Unsigned 64-bit integer storage support
[F64](https://github.com/AndrewSisley/CQLDb/tree/master/cql_storage_types/cql_f64) | [crates.io](https://crates.io/crates/cql_f64) | [docs.rs](https://docs.rs/cql_f64) | 64-bit floating point storage support
[NullableF64](https://github.com/AndrewSisley/CQLDb/tree/master/cql_storage_types/cql_nullable_f64) | [crates.io](https://crates.io/crates/cql_nullable_f64) | [docs.rs](https://docs.rs/cql_nullable_f64) | Nullable 64-bit floating point storage support
[TinyText](https://github.com/AndrewSisley/CQLDb/tree/master/cql_storage_types/cql_tiny_text) | [crates.io](https://crates.io/crates/cql_tiny_text) | [docs.rs](https://docs.rs/cql_tiny_text) | 255 char utf-8 string storage support


## Breaking changes

As this project is a data-storage solution acting upon the file system, some changes may alter the expected structure of the underlying data - preventing one version from correctly utilising a database created on a different version.  Up until version 1.0.0, the minor version (middle number) will be incremented and the breaking version listed in the table below.  If you need to upgrade between one of these versions, I'd suggest reading your entire database from the earlier version and the writing it to a new database with the new target version, I will try and improve this at somepoint.  Please take care.

Crate | Breaking version | Description
--- | --- | ---
[cql_db](https://crates.io/crates/cql_db) | 0.2.0 | Changes made in the key files, and the database file itself. Commits [048e533](https://github.com/AndrewSisley/CQLDb/commit/048e533bb22602a8206a96010b86a387810ab0b2) and [7dcaf7c](https://github.com/AndrewSisley/CQLDb/commit/7dcaf7c9aa2ce7e94c7fbcf0a0e4521944790e3d)


## Getting started

To get started, pick a storage type(s) (examples use U64), and add it as a dependency to your Cargo.toml, along with the core cql_db crate:

```
[dependencies]
//... (any existing dependencies you may have)
cql_db = "^0.2.4"
cql_u64 = "^0.2"
```

You then need to create a folder where you want the database to live, and then try out the below:

```
use std::io::{ Cursor, SeekFrom, Seek };
use cql_db::error::Error;
use cql_u64::{ U64, unpack_stream };

const DATABASE_LOCATION: &str = "PATH_TO_YOUR_DATABASE_DIRECTORY";

pub fn example_cql() -> Result<(), Error> {
    // create a one dimensional database to hold 3 points
    cql_db::create_db::<U64>(
        DATABASE_LOCATION,
        &[3]
    )?;

    // write '1', to [1]
    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &[1],
        1
    )?;

    let mut result = [0; 2];
    let mut stream = Cursor::new(Vec::new());

    // read 2 values from [1] to 'stream'
    cql_db::read_to_stream::<U64>(
        DATABASE_LOCATION,
        &mut stream,
        &[1],
        2
    )?;

    stream.seek(SeekFrom::Start(0)).unwrap();
    unpack_stream(&mut stream, 2, |idx, value| {
        result[idx] = value
    })?;

    assert_eq!(result[0], 1);
    assert_eq!(result[1], 0);
    Ok(())
}
```
More examples can be found in the [rustdocs](https://docs.rs/cql_db).

## Benchmarks

Benchmarks supplied below for the U64 type and are fairly rudimentary (and rounded) and are there to give a rough idea of relative costs.
Full benchmark code can be found in [github](https://github.com/AndrewSisley/CQLDb/tree/master/cql_storage_types/cql_u64) and can be run with `rustup run nightly cargo bench`.  Benchmarks for
other types can be found in the the type's corresponding documentation.

Operation | Database dimensions | Mean time _unchecked (ns) | Mean time (ns)
--- | --- | --- | ---
Single point read | 1 | 2 600 (+/- 300) | 7 500 (+/- 600)
Single point read | 4 | 15 400 (+/- 1 000) | 37 550 (+/- 2 300)
Single point write | 1 | 2 900 (+/- 200) | 7 700 (+/- 400)
Single point write | 4 | 16 000 (+/- 2 000) | 37 700 (+/- 3 000)
Stream read 1 point | 1 | 2 600 (+/- 200) | 10 000 (+/- 850)
Stream read 1 point | 4 | 16 000 (+/- 1 800) | 42 500 (+/- 6 500)
Stream read 50 000 points | 1 | 28 000 000 (+/- 870 000) | 27 630 000 (+/- 180 000)
Stream read 50 000 points | 4 | 28 200 000 (+/- 800 000) | 27 620 000 (+/- 480 000)

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
