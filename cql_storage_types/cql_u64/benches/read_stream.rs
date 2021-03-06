#![feature(test)]
mod constants;
extern crate test;

use std::io::{ Cursor };
use constants::DATABASE_LOCATION;
use test::{ Bencher };
use cql_u64::{ unpack_stream, U64 };
use cql_storage_type_testing_lib::benches::read_stream;

#[bench]
fn _1d_u64_stream_read_location_1_to_1(b: &mut Bencher) {
    let test_fn = read_stream::_1d_read_empty_location_1_to_1::<U64>(DATABASE_LOCATION, &unpack_u64_stream);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _1d_u64_stream_read_location_50000_to_100000(b: &mut Bencher) {
    let test_fn = read_stream::_1d_read_empty_location_50000_to_100000::<U64>(DATABASE_LOCATION, &unpack_u64_stream);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _4d_u64_stream_read_location_1_1_1_1_to_1_1_1_1(b: &mut Bencher) {
    let test_fn = read_stream::_4d_read_empty_location_1_1_1_1_to_1_1_1_1::<U64>(DATABASE_LOCATION, &unpack_u64_stream);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _4d_u64_stream_read_location_1_1_1_50000_to_1_1_1_100000(b: &mut Bencher) {
    let test_fn = read_stream::_4d_read_empty_location_1_1_1_50000_to_1_1_1_100000::<U64>(DATABASE_LOCATION, &unpack_u64_stream);

    b.iter(|| {
        test_fn();
    });
}

fn unpack_u64_stream (stream: &mut Cursor<Vec<u8>>, n_values: usize, result: &mut [u64]) {
    unpack_stream(stream, n_values, |idx, value| {
        result[idx] = value
    }).unwrap()
}
