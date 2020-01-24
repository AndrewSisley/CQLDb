# CQLDb
Ultra lightweight, growable, array-based storage solution, currently limited to storing the following types:
- nullable 64-bit floating point
- TinyText (255 char utf-8 strings)

Currently the project is split into two sub-projects, [cql_storage](cql_storage) and [cql_db](cql_db).  cql_storage contains type specific code used for read/writting specific types from a file, and cql_db contains the logic allowing for database-like manipulation of the file system.

The project works by treating the file system as an N dimensional array, removing the need to scan for items in order to find them. Currently the number of dimensions must be specified on create of the database, however each dimension may grow on demand.

The project was originally built with an eye on storing large volumes of relational time series data, however I am looking to explore other uses in my other projects. Benchmarks were available in the original repo and hopefully I'll clean them up port them over to here pretty promptly.


## Benchmarks

Benchmarks (like everywhere else) are still very much a WIP, however you can find a quick and very rough summary in the table below (run on an 8th gen Intel i5 with SSD). You can run them locally from the cql_db folder with the following command `rustup run nightly cargo bench` if/after you have installed the rust nightly build.

### Single point read (nullable f64)

Database size | Point location | Mean read time (ns)
--- | --- | ---
[1] | [1] | 2 218 (+/- 662)
[100 000] | [100 000] | 2 239 (+/- 488)
[1, 1, 1, 1] | [1, 1, 1, 1] | 11 481 (+/- 1 272)
[1, 100 000, 1, 1] | [1, 100 000, 1, 1] | 11 471 (+/- 1 052)
[1, 1, 1, 100 000] | [1, 1, 1, 100 000] | 11 441 (+/- 1 830)
