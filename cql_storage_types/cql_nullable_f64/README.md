# CQL NullableF64
This crate implements various [CqlType](https://docs.rs/cql_model/0.2/cql_model/trait.CqlType.html) derivatives for storing `Option<f64>` values in a CQL database.

Will allocate 9 bytes per value [linked](https://docs.rs/cql_db/0.2/cql_db/fn.link_dimensions.html).

## Benchmarks
Benchmarks supplied below are fairly rudimentary (and rounded) and are there to give a rough idea of relative costs.
Full benchmark code can be found in [github](https://github.com/AndrewSisley/CQLDb/tree/master/cql_storage_types/cql_nullable_f64) and can be run with
`rustup run nightly cargo bench`.

Operation | Database dimensions | Mean time (ns)
--- | --- | ---
Single point read | 1 | 3 100 (+/- 300)
Single point read | 4 | 16 100 (+/- 2 200)
Single point write | 1 | 2 900 (+/- 300)
Single point write | 4 | 15 700 (+/- 1 000)
Stream read 1 point | 1 | 2 600 (+/- 300)
Stream read 1 point | 4 | 15 800 (+/- 2 000)
Stream read 50 000 points | 1 | 28 000 000 (+/- 1 000 000)
Stream read 50 000 points | 4 | 27 900 000 (+/- 80 000)

## Getting started
To get started, add the below dependencies to your Cargo.toml:

```
[dependencies]
//... (any existing dependencies you may have)
cql_db = "^0.2.4"
cql_nullable_f64 = "^0.2"
```

Then need to create a folder where you want the database to live, and then try out the below:

```
use std::io::{ Cursor, SeekFrom, Seek };
use std::error::Error;
use cql_nullable_f64::{ NullableF64, unpack_stream };

const DATABASE_LOCATION: &str = "PATH_TO_YOUR_DATABASE_DIRECTORY";

pub fn example_cql() -> Result<(), Error> {
    // create a one dimensional database to hold 3 points
    cql_db::create_db::<NullableF64>(
        DATABASE_LOCATION,
        &[3]
    )?;

    // write '-1.6', to [1]
    cql_db::write_value::<NullableF64>(
        DATABASE_LOCATION,
        &[1],
        Some(-1.6)
    )?;

    // write '5.4', to [3]
    cql_db::write_value::<NullableF64>(
        DATABASE_LOCATION,
        &[3],
        Some(5.4)
    )?;

    let mut result: [Option<f64>; 3] = [None; 3];
    let mut stream = Cursor::new(Vec::new());

    // read 3 values from [1] to 'stream'
    cql_db::read_to_stream::<NullableF64>(
        DATABASE_LOCATION,
        &mut stream,
        &[1],
        3
    )?;

    stream.seek(SeekFrom::Start(0)).unwrap();
    unpack_stream(&mut stream, 3, |idx, value| {
        result[idx] = value
    })?;

    assert_eq!(result[0], Some(-1.6));
    assert_eq!(result[1], None);
    assert_eq!(result[2], Some(5.4));
    Ok(())
}
```

## More info
For further information and more examples, please see the [rustdocs](https://docs.rs/cql_nullable_f64).  Additional storage types are documented in the [cql_db](https://crates.io/crates/cql_db) crate.
