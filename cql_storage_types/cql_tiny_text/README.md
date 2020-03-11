# CQL TinyText
This crate implements various [CqlType](https://docs.rs/cql_model/0.2/cql_model/trait.CqlType.html) derivatives for storing String values of up to (and including) 255 chars in a
[CQL database](https://docs.rs/cql_db/0.2/cql_db/).

Will allocate 1020 bytes per value [linked](https://docs.rs/cql_db/0.2/cql_db/fn.link_dimensions.html).

## Benchmarks
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

## Getting started
To get started, add the below dependencies to your Cargo.toml:

```
[dependencies]
//... (any existing dependencies you may have)
cql_db = "^0.2.4"
cql_tiny_text = "^0.2"
```

Then need to create a folder where you want the database to live, and then try out the below:

```
use std::convert::TryFrom;
use std::io::{ Cursor, SeekFrom, Seek };
use std::error::Error;
use cql_tiny_text::{ TinyText, unpack_stream };

const DATABASE_LOCATION: &str = "PATH_TO_YOUR_DATABASE_DIRECTORY";

pub fn example_cql() -> Result<(), Box<dyn Error>> {
    let value1 = "item one";
    let value3 = "شماره ۳";

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[3]
    )?;

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &[1],
        TinyText::try_from(value1)?
    )?;

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &[3],
        TinyText::try_from(value3)?
    )?;

    let mut result = Vec::with_capacity(3);
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream::<TinyText>(
        DATABASE_LOCATION,
        &mut stream,
        &[1],
        3
    )?;

    stream.seek(SeekFrom::Start(0));
    unpack_stream(&mut stream, 3, |_, value| {
        result.push(value)
    })?;

    assert_eq!(result[0], TinyText::try_from(value1)?);
    assert_eq!(result[1], TinyText::new());
    assert_eq!(result[2], TinyText::try_from(value3)?);
    Ok(())
}
```

## More info
For further information and more examples, please see the [rustdocs](https://docs.rs/cql_tiny_text).  Additional storage types are documented in the [cql_db](https://crates.io/crates/cql_db) crate.
