# CQLDb
Lightweight, growable, array-based storage solution, currently with the out-of-the-box storage types (custom types also possible):
- [U64](cql_storage_types/cql_u64) (unsigned 64 integers)
- [F64](cql_storage_types/cql_f64) (64-bit floating point)
- [NullableF64](cql_storage_types/cql_nullable_f64) (nullable 64-bit floating point)
- [TinyText](cql_storage_types/cql_tiny_text) (255 char utf-8 strings)

The project works by treating the file system as an N dimensional array, removing the need to scan for items in order to find them. Currently the number of dimensions must be specified on create of the database, however each dimension (bar the last) may grow on demand.

The project was originally built with an eye on storing large volumes of relational time series data, however I am looking to explore other uses in my other projects.


## Project structure

The project is split into two core sub-projects, [cql_db](cql_db) and [cql_model](cql_model), and a sub-project per supported type within the [cql_storage_types](cql_storage_types) folder.

The [cql_db](cql_db) sub-project contains the core logic orchestrating the type specific logic, and the array-based logic allowing the whole thing to function as a database.  It is dependent on [cql_model](cql_model) and the [U64](cql_storage_types/cql_u64) type.

The [cql_model](cql_model) sub-project contains the interfaces consumed by [cql_db](cql_db) and the [storage types](cql_storage_types) and is referenced by all sub-projects.

The [storage type](cql_storage_types) specific projects contains type specific code used for read/writting specific types from a file, implementing the interfaces within the [cql_model](cql_model) sub-project.

To use this project you'll need to import the [cql_db](cql_db) sub-project and either each of the [storage types](cql_storage_types) that you with to use, or the [cql_model](cql_model) and you own implementations of the traits within - should you wish to use other types.

Rustdocs (with examples) and crates for all published components can be found in the table below:

Repo link |Crate | Documentation | Description
--- | --- | --- | ---
[CQL Db](cql_db) | [crates.io](https://crates.io/crates/cql_db) | [docs.rs](https://docs.rs/cql_db) | Core CQL database engine
[CQL Model](cql_model) | [crates.io](https://crates.io/crates/cql_model) | [docs.rs](https://docs.rs/cql_model) | Core CQL database models/interfaces
[U64](cql_storage_types/cql_u64) | [crates.io](https://crates.io/crates/cql_u64) | [docs.rs](https://docs.rs/cql_u64) | Unsigned 64-bit interger storage support
[F64](cql_storage_types/cql_f64) | *unpublished* | *unpublished* | 64-bit floating point storage support
[NullableF64](cql_storage_types/cql_nullable_f64) | [crates.io](https://crates.io/crates/cql_nullable_f64) | [docs.rs](https://docs.rs/cql_nullable_f64) | Nullable 64-bit floating point storage support
[TinyText](cql_storage_types/cql_tiny_text) | [crates.io](https://crates.io/crates/cql_tiny_text) | [docs.rs](https://docs.rs/cql_tiny_text) | 255 char utf-8 string storage support


## Quick note on safety

CQL Db currently performs next to no parameter checking, and has very little deliberate error handling.  For example, calling create_db with the directory of an existing database will replace the existing one, and read_to_stream will happily wrap itself around the bounds of it's requested location if you ask it to read more points than are available.  I plan on improving this soon (probably preserving the unsafe methods if the benchmarks take a hit), but have not yet done so yet, so take care.


## Getting started

To get started, pick a storage type(s) (examples use NullableF64), and add it as a dependency to your Cargo.toml, along with the core cql_db crate:

```
[dependencies]
//... (any existing dependencies you may have)
cql_db = "0.1.*"
cql_nullable_f64 = "0.1.*"
```

You then need to create a folder where you want the database to live, and then try out the below:

```
use std::io::{ Cursor, SeekFrom, Seek };
use cql_nullable_f64::{ NullableF64, unpack_stream };

const DATABASE_LOCATION: &str = "PATH_TO_YOUR_DATABASE_DIRECTORY";
const N_VALUES_TO_READ: usize = 3;

pub fn example_cql() {
    let base_point = [0];
    let value1 = Some(-1.6);
    let value3 = Some(5.4);

    // creates a one dimensional database, with a capacity of 3
    cql_db::create_db::<NullableF64>(
        DATABASE_LOCATION,
        &[3]
    );

    // writes Some(-1.6) to [0]
    cql_db::write_value::<NullableF64>(
        DATABASE_LOCATION,
        &base_point,
        value1
    );

    // writes Some(5.4) to [2]
    cql_db::write_value::<NullableF64>(
        DATABASE_LOCATION,
        &[base_point[0] + 2],
        value3
    );

    let mut result: [Option<f64>; N_VALUES_TO_READ] = [None; N_VALUES_TO_READ];
    let mut stream = Cursor::new(Vec::new());

    // reads 3 points from [0] into `stream`
    cql_db::read_to_stream::<NullableF64>(
        DATABASE_LOCATION,
        &mut stream,
        &base_point,
        N_VALUES_TO_READ as u64
    );

    // returns to the start of the stream
    stream.seek(SeekFrom::Start(0)).unwrap();
    // unpacks the stream value by value into the result[]
    unpack_stream(&mut stream, N_VALUES_TO_READ, |idx, value| {
        result[idx] = value
    });

    assert_eq!(result[0], value1);
    assert_eq!(result[1], None);
    assert_eq!(result[2], value3);
}
```
More examples can be found in the [rustdocs](https://docs.rs/cql_db).

## Benchmarks

Benchmarks (like everywhere else) are still very much a WIP, however you can find a quick and very rough summary in the table below (run on an 8th gen Intel i5 with SSD). You can run them locally from the [NullableF64](cql_storage_types/cql_nullable_f64) folder with the following command `rustup run nightly cargo bench` if/after you have installed the rust nightly build, it will use about 900 kB of disk space.

### Single point read - NullableF64

Database size | Point location | Mean read time (ns)
--- | --- | ---
[1] | [1] | 2 187 (+/- 268)
[100 000] | [100 000] | 2 197 (+/- 439)
[1, 1, 1, 1] | [1, 1, 1, 1] | 11 416 (+/- 1 111)
[1, 100 000, 1, 1] | [1, 100 000, 1, 1] | 11 487 (+/- 2 755)
[1, 1, 1, 100 000] | [1, 1, 1, 100 000] | 11 468 (+/- 1 678)

### Stream read - NullableF64
N number of points read across the last axis from the start location, benchmark includes the time taken to read from bytestream into an array.

Database size | Some/None | Start location | N points | Mean read time (ns) | Mean per point (ns)
--- | --- | --- | --- | --- | ---
[1] | None | [1] | 1 | 1 849 (+/- 360) | 1 849
[1] | Some | [1] | 1 | 1 899 (+/- 374) | 1 899
[100 000] | None | [50 000] | 50 000 | 16 159 340 (+/- 576 181) | 323
[100 000] | Some | [50 000] | 50 000 | 19 036 834 (+/- 338 129) | 381
[1, 1, 1, 1] | None | [1, 1, 1, 1] | 1 | 11 303 (+/- 2 576) | 11 303
[1, 1, 1, 1] | Some | [1, 1, 1, 1] | 1 | 11 276 (+/- 3 574) | 11 276
[1, 1, 1, 100 000] | None | [1, 1, 1, 50 000] | 50 000 | 18 802 770 (+/- 33 520) | 376
[1, 1, 1, 100 000] | Some | [1, 1, 1, 50 000] | 50 000 | 18 796 934 (+/- 71 134) | 376

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
