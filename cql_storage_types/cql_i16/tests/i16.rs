mod constants;

use serial_test::serial;
use std::io::{ Cursor };
use constants::DATABASE_LOCATION;
use cql_i16::{ I16, unpack_stream };
use cql_storage_type_testing_lib::tests::{ read_write_single_unchecked, read_write_stream_unchecked };
pub mod single_point_read_writes;

#[test]
#[serial]
fn _4d_i16_database_allows_for_single_point_read_writes_given_multiple_values_and_overwrites() {
    read_write_single_unchecked::_4d_database_allows_for_single_point_read_writes_given_multiple_values_and_overwrites::<I16>(
        DATABASE_LOCATION,
        5,
        -20,
        0,
        30000
    );
}

#[test]
#[serial]
fn _1d_i16_database_allows_for_stream_reads() {
    read_write_stream_unchecked::_1d_database_allows_for_stream_reads::<I16, &dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [i16])>(
        DATABASE_LOCATION,
        42,
        16,
        80,
        &unpack_i16_stream
    );
}

#[test]
#[serial]
fn _4d_i16_database_allows_for_stream_reads() {
    read_write_stream_unchecked::_4d_database_allows_for_stream_reads::<I16, &dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [i16])>(
        DATABASE_LOCATION,
        42,
        16,
        80,
        &unpack_i16_stream
    );
}

fn unpack_i16_stream (stream: &mut Cursor<Vec<u8>>, n_values: usize, result: &mut [i16]) {
    unpack_stream(stream, n_values, |idx, value| {
        result[idx] = value
    }).unwrap()
}
