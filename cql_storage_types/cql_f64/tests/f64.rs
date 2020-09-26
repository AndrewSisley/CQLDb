mod constants;

use serial_test::serial;
use std::io::{ Cursor };
use constants::DATABASE_LOCATION;
use cql_f64::{ F64, unpack_stream };
use cql_storage_type_testing_lib::tests;

#[test]
#[serial]
fn _1d_f64_database_allows_for_single_point_read_writes() {
    tests::_1d_database_allows_for_single_point_read_writes::<F64>(
        DATABASE_LOCATION,
        42.4
    );
}

#[test]
#[serial]
fn _4d_f64_database_allows_for_single_point_read_writes() {
    tests::_4d_database_allows_for_single_point_read_writes::<F64>(
        DATABASE_LOCATION,
        5.1
    );
}

#[test]
#[serial]
fn _4d_f64_database_allows_for_single_point_read_writes_given_multiple_values_and_overwrites() {
    tests::_4d_database_allows_for_single_point_read_writes_given_multiple_values_and_overwrites::<F64>(
        DATABASE_LOCATION,
        5.1,
        -20.0,
        0.56,
        30000.3
    );
}

#[test]
#[serial]
fn _1d_f64_database_allows_for_stream_reads() {
    tests::_1d_database_allows_for_stream_reads::<F64, &dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [f64])>(
        DATABASE_LOCATION,
        42.423525,
        16.1,
        80.8,
        &unpack_f64_stream
    );
}

#[test]
#[serial]
fn _4d_f64_database_allows_for_stream_reads() {
    tests::_4d_database_allows_for_stream_reads::<F64, &dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [i16])>(
        DATABASE_LOCATION,
        42.423525,
        16.1,
        80.8,
        &unpack_f64_stream
    );
}

fn unpack_f64_stream (stream: &mut Cursor<Vec<u8>>, n_values: usize, result: &mut [f64]) {
    unpack_stream(stream, n_values, |idx, value| {
        result[idx] = value
    }).unwrap()
}
