# CQL I16
This crate implements various [CqlType](https://docs.rs/cql_model/0.2/cql_model/trait.CqlType.html) derivatives for storing `i16` values in a CQL database.

Will allocate 2 bytes per value [linked](https://docs.rs/cql_db/0.2/cql_db/fn.link_dimensions.html).

## Benchmarks
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

## Getting started
To get started, add the below dependencies to your Cargo.toml:

```
[dependencies]
//... (any existing dependencies you may have)
cql_db = "^0.2.4"
cql_i16 = "^0.2"
```

Then need to create a folder where you want the database to live, and then try out the below:

```
use std::io::{ Cursor, SeekFrom, Seek };
use cql_db::error::Error;
use cql_i16::{ I16, unpack_stream };

const DATABASE_LOCATION: &str = "PATH_TO_YOUR_DATABASE_DIRECTORY";

pub fn example_cql() -> Result<(), Error> {
    // create a one dimensional database to hold 3 points
    cql_db::create_db::<I16>(
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
    cql_db::read_to_stream::<I16>(
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

## More info
For further information and more examples, please see the [rustdocs](https://docs.rs/cql_i16).  Additional storage types are documented in the [cql_db](https://crates.io/crates/cql_db) crate.
