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
    cql_storage_type_testing_lib::_4d_database_allows_for_single_point_read_writes_given_multiple_values_and_overwrites::<NullableF64>(
        DATABASE_LOCATION,
        Some(-5.6),
        Some(20.61241),
        Some(0f64),
        Some(-5745.6642)
    );
}

#[test]
#[serial]
fn _1d_f64_nullable_database_allows_for_stream_reads() {
    cql_storage_type_testing_lib::_1d_database_allows_for_stream_reads::<NullableF64, &dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [Option<f64>])>(
        DATABASE_LOCATION,
        Some(42.3),
        Some(-414.16),
        Some(8.3),
        &unpack_nullable_f64_stream
    );
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

fn unpack_nullable_f64_stream (stream: &mut Cursor<Vec<u8>>, n_values: usize, result: &mut [Option<f64>]) {
    unpack_stream(stream, n_values, |idx, value| {
        result[idx] = value
    }).unwrap()
}
