# CQLDb
Ultra lightweight, growable, array-based storage solution, currently limited to storing nullable f64s at the moment.

Currently the project is split into two sub-projects, [cql_storage] and [cql_db].  cql_storage contains type specific code used for read/writting specific types from a file, and cql_db contains the logic allowing for database-like manipulation of the file system.

The project works by treating the file system as an N dimensional array, removing the need to scan for items in order to find them. Currently the number of dimensions must be specified on create of the database, however each dimension may grow on demand.

The project was originally built with an eye on storing large volumes of relational time series data, however I am looking to explore other uses in my other projects. Benchmarks were available in the original repo and hopefully I'll clean them up port them over to here pretty promptly.
