#![allow(non_snake_case)]

mod constants;

use serial_test::serial;
use std::io::{ Cursor, SeekFrom, Seek };

use constants::DATABASE_LOCATION;
use cql_u64::{ U64, unpack_stream };
use cql_db::error;

#[test]
#[serial]
fn read_to_stream__returns_DimensionsOutOfRangeError__given_1d_u64_database_and_0d_location() {
    let db_dimensions = [1];

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    ).unwrap();

    let location = [];
    let mut stream = Cursor::new(Vec::new());

    let result = match cql_db::read_to_stream::<U64>(
        DATABASE_LOCATION,
        &mut stream,
        &location,
        1
    ) {
        Err(error) => match error {
            error::Error::Cql(cql_error) => match cql_error {
                error::cql::Error::DimensionsOutOfRangeError{ requested, min, max } => Some((requested, min, max)),
                _ => None,
            },
            _ => None,
        }
        _ => None,
    };

    assert_eq!(
        result.unwrap(),
        (location.len(), db_dimensions.len(), db_dimensions.len())
    );
}

#[test]
#[serial]
fn read_to_stream__returns_DimensionsOutOfRangeError__given_1d_u64_database_and_2d_location() {
    let db_dimensions = [1];

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    ).unwrap();

    let location = [1, 1];
    let mut stream = Cursor::new(Vec::new());

    let result = match cql_db::read_to_stream::<U64>(
        DATABASE_LOCATION,
        &mut stream,
        &location,
        1
    ) {
        Err(error) => match error {
            error::Error::Cql(cql_error) => match cql_error {
                error::cql::Error::DimensionsOutOfRangeError{ requested, min, max } => Some((requested, min, max)),
                _ => None,
            },
            _ => None,
        }
        _ => None,
    };

    assert_eq!(
        result.unwrap(),
        (location.len(), db_dimensions.len(), db_dimensions.len())
    );
}

#[test]
#[serial]
fn read_to_stream__returns_DimensionsOutOfRangeError__given_2d_u64_database_and_1d_location() {
    let db_dimensions = [1, 1];

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    ).unwrap();

    let location = [1];
    let mut stream = Cursor::new(Vec::new());

    let result = match cql_db::read_to_stream::<U64>(
        DATABASE_LOCATION,
        &mut stream,
        &location,
        1
    ) {
        Err(error) => match error {
            error::Error::Cql(cql_error) => match cql_error {
                error::cql::Error::DimensionsOutOfRangeError{ requested, min, max } => Some((requested, min, max)),
                _ => None,
            },
            _ => None,
        }
        _ => None,
    };

    assert_eq!(
        result.unwrap(),
        (location.len(), db_dimensions.len(), db_dimensions.len())
    );
}

#[test]
#[serial]
fn read_to_stream__returns_DimensionsOutOfRangeError__given_2d_u64_database_and_3d_location() {
    let db_dimensions = [1, 1];

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    ).unwrap();

    let location = [1, 1, 1];
    let mut stream = Cursor::new(Vec::new());

    let result = match cql_db::read_to_stream::<U64>(
        DATABASE_LOCATION,
        &mut stream,
        &location,
        1
    ) {
        Err(error) => match error {
            error::Error::Cql(cql_error) => match cql_error {
                error::cql::Error::DimensionsOutOfRangeError{ requested, min, max } => Some((requested, min, max)),
                _ => None,
            },
            _ => None,
        }
        _ => None,
    };

    assert_eq!(
        result.unwrap(),
        (location.len(), db_dimensions.len(), db_dimensions.len())
    );
}

#[test]
#[serial]
fn read_to_stream__returns_IndexOutOfRangeError__given_1d_u64_database_and_zero_index() {
    let db_dimensions = [1];

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    ).unwrap();

    let small_index = 0;
    let location = [0];
    let mut stream = Cursor::new(Vec::new());

    let result = match cql_db::read_to_stream::<U64>(
        DATABASE_LOCATION,
        &mut stream,
        &location,
        1
    ) {
        Err(error) => match error {
            error::Error::Cql(cql_error) => match cql_error {
                error::cql::Error::IndexOutOfRangeError{ dimension_index, requested, min, max } => Some((dimension_index, requested, min, max)),
                _ => None,
            },
            _ => None,
        }
        _ => None,
    };

    assert_eq!(
        result.unwrap(),
        (small_index, location[small_index], 1, db_dimensions[small_index])
    );
}

#[test]
#[serial]
fn read_to_stream__returns_IndexOutOfRangeError__given_2d_u64_database_and_one_zero_index() {
    let db_dimensions = [1, 1];

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    ).unwrap();

    let small_index = 1;
    let location = [1, 0];
    let mut stream = Cursor::new(Vec::new());

    let result = match cql_db::read_to_stream::<U64>(
        DATABASE_LOCATION,
        &mut stream,
        &location,
        1
    ) {
        Err(error) => match error {
            error::Error::Cql(cql_error) => match cql_error {
                error::cql::Error::IndexOutOfRangeError{ dimension_index, requested, min, max } => Some((dimension_index, requested, min, max)),
                _ => None,
            },
            _ => None,
        }
        _ => None,
    };

    assert_eq!(
        result.unwrap(),
        (small_index, location[small_index], 1, db_dimensions[small_index])
    );
}

#[test]
#[serial]
fn read_to_stream__returns_IndexOutOfRangeError__given_2d_u64_database_and_zero_one_index() {
    let db_dimensions = [1, 1];

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    ).unwrap();

    let small_index = 0;
    let location = [0, 1];
    let mut stream = Cursor::new(Vec::new());

    let result = match cql_db::read_to_stream::<U64>(
        DATABASE_LOCATION,
        &mut stream,
        &location,
        1
    ) {
        Err(error) => match error {
            error::Error::Cql(cql_error) => match cql_error {
                error::cql::Error::IndexOutOfRangeError{ dimension_index, requested, min, max } => Some((dimension_index, requested, min, max)),
                _ => None,
            },
            _ => None,
        }
        _ => None,
    };

    assert_eq!(
        result.unwrap(),
        (small_index, location[small_index], 1, db_dimensions[small_index])
    );
}

#[test]
#[serial]
fn read_to_stream__returns_IndexOutOfRangeError__given_2d_u64_database_and_zero_one_index_and_too_large_n_values() {
    let db_dimensions = [1, 1];

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    ).unwrap();

    let location = [1, 1];
    let n_values_to_read = 2;
    let mut stream = Cursor::new(Vec::new());

    let result = match cql_db::read_to_stream::<U64>(
        DATABASE_LOCATION,
        &mut stream,
        &location,
        n_values_to_read
    ) {
        Err(error) => match error {
            error::Error::Cql(cql_error) => match cql_error {
                error::cql::Error::IndexOutOfRangeError{ dimension_index, requested, min, max } => Some((dimension_index, requested, min, max)),
                _ => None,
            },
            _ => None,
        }
        _ => None,
    };

    let last_index = location.len() - 1;
    assert_eq!(
        result.unwrap(),
        (last_index, location[last_index] + n_values_to_read - 1, 1, db_dimensions[last_index])
    );
}

#[test]
#[serial]
#[cfg(feature = "matrix")]
fn _4d_u64_database_allows_for_stream_reads() {
    let base_point = [1, 1, 1, 2];
    const N_VALUES_TO_READ: usize = 3;
    let value1 = 42;
    let value2 = 16;
    let value3 = 80;

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[1, 1, 1, 10]
    ).unwrap();

    cql_db::link_dimensions::<U64>(
        DATABASE_LOCATION,
        &base_point[0..3]
    ).unwrap();

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &base_point,
        value1
    ).unwrap();

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &[1, 1, 1, base_point[3] + 1],
        value2
    ).unwrap();

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &[1, 1, 1, base_point[3] + 2],
        value3
    ).unwrap();

    let mut result = [0; N_VALUES_TO_READ];
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream_unchecked::<U64>(
        DATABASE_LOCATION,
        &mut stream,
        &base_point,
        N_VALUES_TO_READ as u64
    ).unwrap();

    stream.seek(SeekFrom::Start(0)).unwrap();

    unpack_stream(&mut stream, N_VALUES_TO_READ, |idx, value| {
        result[idx] = value
    }).unwrap();

    assert_eq!(result[0], value1);
    assert_eq!(result[1], value2);
    assert_eq!(result[2], value3);
}

#[test]
#[serial]
#[cfg(feature = "matrix")]
fn _4d_u64_database_allows_for_stream_reads_checked() {
    let base_point = [1, 1, 1, 2];
    const N_VALUES_TO_READ: usize = 3;
    let value1 = 42;
    let value2 = 16;
    let value3 = 80;

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[1, 1, 1, 10]
    ).unwrap();

    cql_db::link_dimensions::<U64>(
        DATABASE_LOCATION,
        &base_point[0..3]
    ).unwrap();

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &base_point,
        value1
    ).unwrap();

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &[1, 1, 1, base_point[3] + 1],
        value2
    ).unwrap();

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &[1, 1, 1, base_point[3] + 2],
        value3
    ).unwrap();

    let mut result = [0; N_VALUES_TO_READ];
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream::<U64>(
        DATABASE_LOCATION,
        &mut stream,
        &base_point,
        N_VALUES_TO_READ as u64
    ).unwrap();

    stream.seek(SeekFrom::Start(0)).unwrap();

    unpack_stream(&mut stream, N_VALUES_TO_READ, |idx, value| {
        result[idx] = value
    }).unwrap();

    assert_eq!(result[0], value1);
    assert_eq!(result[1], value2);
    assert_eq!(result[2], value3);
}
