# CQLDb
Ultra lightweight, growable, array-based storage solution, currently with the out-of-the-box storage types (custom types also possible):
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
