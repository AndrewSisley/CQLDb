mod constants;

use serial_test::serial;
use std::io::{ Cursor };
use constants::DATABASE_LOCATION;
use cql_nullable_f64::{ NullableF64, unpack_stream };
use cql_storage_type_testing_lib::tests;

#[test]
#[serial]
fn _1d_f64_nullable_database_allows_for_single_point_read_writes() {
    tests::_1d_database_allows_for_single_point_read_writes::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.87)
    );
}

#[test]
#[serial]
fn _4d_f64_nullable_database_allows_for_single_point_read_writes() {
    tests::_4d_database_allows_for_single_point_read_writes::<NullableF64>(
        DATABASE_LOCATION,
        Some(-5.6)
    );
}

#[test]
#[serial]
fn _4d_f64_nullable_database_allows_for_single_point_read_writes_given_multiple_values_and_overwrites() {
    tests::_4d_database_allows_for_single_point_read_writes_given_multiple_values_and_overwrites::<NullableF64>(
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
    tests::_1d_database_allows_for_stream_reads::<NullableF64, &dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [Option<f64>])>(
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
    tests::_4d_database_allows_for_stream_reads::<NullableF64, &dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [Option<f64>])>(
        DATABASE_LOCATION,
        Some(4.2),
        Some(1124.6),
        Some(-0.80),
        &unpack_nullable_f64_stream
    );
}

fn unpack_nullable_f64_stream (stream: &mut Cursor<Vec<u8>>, n_values: usize, result: &mut [Option<f64>]) {
    unpack_stream(stream, n_values, |idx, value| {
        result[idx] = value
    }).unwrap()
}
