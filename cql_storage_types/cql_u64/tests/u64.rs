mod constants;

use serial_test::serial;
use std::io::{ Cursor, SeekFrom, Seek };
use constants::DATABASE_LOCATION;
use cql_u64::{ U64, unpack_stream };

#[test]
#[serial]
fn _1d_u64_database_allows_for_single_point_read_writes() {
    cql_storage_type_testing_lib::_1d_database_allows_for_single_point_read_writes::<U64>(
        DATABASE_LOCATION,
        42
    );
}

#[test]
#[serial]
fn _4d_u64_database_allows_for_single_point_read_writes() {
    cql_storage_type_testing_lib::_4d_database_allows_for_single_point_read_writes::<U64>(
        DATABASE_LOCATION,
        5
    );
}

#[test]
#[serial]
fn _4d_u64_database_allows_for_single_point_read_writes_given_multiple_values_and_overwrites() {
    cql_storage_type_testing_lib::_4d_database_allows_for_single_point_read_writes_given_multiple_values_and_overwrites::<U64>(
        DATABASE_LOCATION,
        5,
        20,
        0,
        9999999999999
    );
}

#[test]
#[serial]
fn _1d_u64_database_allows_for_stream_reads() {
    let base_point = [2];
    const N_VALUES_TO_READ: usize = 3;
    let value1 = 42;
    let value2 = 16;
    let value3 = 80;

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[10]
    ).unwrap();

    cql_db::write_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &base_point,
        value1
    ).unwrap();

    cql_db::write_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &[base_point[0] + 1],
        value2
    ).unwrap();

    cql_db::write_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &[base_point[0] + 2],
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

    cql_db::link_dimensions_unchecked::<U64>(
        DATABASE_LOCATION,
        &base_point[0..3]
    ).unwrap();

    cql_db::write_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &base_point,
        value1
    ).unwrap();

    cql_db::write_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &[1, 1, 1, base_point[3] + 1],
        value2
    ).unwrap();

    cql_db::write_value_unchecked::<U64>(
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
