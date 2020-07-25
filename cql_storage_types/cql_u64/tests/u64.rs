mod constants;

use serial_test::serial;
use std::io::{ Cursor };
use constants::DATABASE_LOCATION;
use cql_u64::{ U64, unpack_stream };
use cql_storage_type_testing_lib::tests;

#[test]
#[serial]
fn _1d_u64_database_allows_for_single_point_read_writes() {
    tests::_1d_database_allows_for_single_point_read_writes::<U64>(
        DATABASE_LOCATION,
        42
    );
}

#[test]
#[serial]
fn _4d_u64_database_allows_for_single_point_read_writes() {
    tests::_4d_database_allows_for_single_point_read_writes::<U64>(
        DATABASE_LOCATION,
        5
    );
}

#[test]
#[serial]
fn _4d_u64_database_allows_for_single_point_read_writes_given_multiple_values_and_overwrites() {
    tests::_4d_database_allows_for_single_point_read_writes_given_multiple_values_and_overwrites::<U64>(
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
    tests::_1d_database_allows_for_stream_reads::<U64, &dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [u64])>(
        DATABASE_LOCATION,
        42,
        16,
        80,
        &unpack_u64_stream
    );
}

#[test]
#[serial]
fn _4d_u64_database_allows_for_stream_reads() {
    tests::_4d_database_allows_for_stream_reads::<U64, &dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [u64])>(
        DATABASE_LOCATION,
        42,
        16,
        80,
        &unpack_u64_stream
    );
}

fn unpack_u64_stream (stream: &mut Cursor<Vec<u8>>, n_values: usize, result: &mut [u64]) {
    unpack_stream(stream, n_values, |idx, value| {
        result[idx] = value
    }).unwrap()
}
