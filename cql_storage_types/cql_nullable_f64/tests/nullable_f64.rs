mod constants;

use serial_test::serial;
use std::io::{ Cursor, SeekFrom, Seek };
use constants::DATABASE_LOCATION;
use cql_nullable_f64::{ NullableF64, unpack_stream };

#[test]
#[serial]
fn _1d_f64_nullable_database_allows_for_single_point_read_writes() {
    cql_storage_type_testing_lib::_1d_database_allows_for_single_point_read_writes::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.87)
    );
}

#[test]
#[serial]
fn _4d_f64_nullable_database_allows_for_single_point_read_writes() {
    cql_storage_type_testing_lib::_4d_database_allows_for_single_point_read_writes::<NullableF64>(
        DATABASE_LOCATION,
        Some(-5.6)
    );
}

#[test]
#[serial]
fn _4d_f64_nullable_database_allows_for_single_point_read_writes_given_multiple_values_and_overwrites() {
    let point1 = [2, 4, 3, 1];
    let point2 = [1, 4, 3, 1];
    let point3 = [2, 1, 3, 1];
    let point4 = [2, 4, 3, 2];
    let value1 = Some(-5.6);
    let value2 = Some(20.61241);
    let value3 = Some(0f64);
    let value5 = Some(-5745.6642);

    cql_db::create_db_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &[2, 5, 3, 4]
    ).unwrap();

    cql_db::link_dimensions_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &point1[0..3],
    ).unwrap();

    cql_db::link_dimensions_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &point2[0..3],
    ).unwrap();

    cql_db::link_dimensions_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &point3[0..3],
    ).unwrap();

    cql_db::write_value_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &point1,
        value1
    ).unwrap();

    cql_db::write_value_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &point2,
        value2
    ).unwrap();

    cql_db::write_value_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &point3,
        value3
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &point1
    ).unwrap();

    let result2 = cql_db::read_value_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &point2
    ).unwrap();

    let result3 = cql_db::read_value_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &point3
    ).unwrap();

    let result4 = cql_db::read_value_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &point4
    ).unwrap();

    assert_eq!(result1, value1);
    assert_eq!(result2, value2);
    assert_eq!(result3, value3);
    assert_eq!(result4, None);

    cql_db::write_value_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &point2,
        value5
    ).unwrap();

    let result5 = cql_db::read_value_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &point2
    ).unwrap();

    assert_eq!(result5, value5);
}

#[test]
#[serial]
fn _1d_f64_nullable_database_allows_for_stream_reads() {
    let base_point = [2];
    const N_VALUES_TO_READ: usize = 4;
    let value1 = Some(42.3);
    let value2 = Some(-414.16);
    let value4 = Some(8.3);

    cql_db::create_db_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &[10]
    ).unwrap();

    cql_db::write_value_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &base_point,
        value1
    ).unwrap();

    cql_db::write_value_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &[base_point[0] + 1],
        value2
    ).unwrap();

    cql_db::write_value_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &[base_point[0] + 3],
        value4
    ).unwrap();

    let mut result: [Option<f64>; N_VALUES_TO_READ] = [None; N_VALUES_TO_READ];
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream_unchecked::<NullableF64>(
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
    assert_eq!(result[2], None);
    assert_eq!(result[3], value4);
}

#[test]
#[serial]
fn _4d_f64_nullable_database_allows_for_stream_reads() {
    let base_point = [1, 1, 1, 2];
    const N_VALUES_TO_READ: usize = 4;
    let value1 = Some(4.2);
    let value2 = Some(1124.6);
    let value4 = Some(-0.80);

    cql_db::create_db_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &[1, 1, 1, 10]
    ).unwrap();

    cql_db::link_dimensions_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &base_point[0..3]
    ).unwrap();

    cql_db::write_value_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &base_point,
        value1
    ).unwrap();

    cql_db::write_value_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &[1, 1, 1, base_point[3] + 1],
        value2
    ).unwrap();

    cql_db::write_value_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &[1, 1, 1, base_point[3] + 3],
        value4
    ).unwrap();

    let mut result: [Option<f64>; N_VALUES_TO_READ] = [None; N_VALUES_TO_READ];
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream_unchecked::<NullableF64>(
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
    assert_eq!(result[2], None);
    assert_eq!(result[3], value4);
}
