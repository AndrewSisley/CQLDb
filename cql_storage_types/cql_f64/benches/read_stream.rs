#![feature(test)]
mod constants;
extern crate test;

use std::io::{ Cursor };
use constants::DATABASE_LOCATION;
use test::{ Bencher };
use cql_f64::{ unpack_stream, F64 };
use cql_storage_type_testing_lib::benches::read_stream;

#[bench]
fn _1d_f64_stream_read_location_1_to_1(b: &mut Bencher) {
    let test_fn = read_stream::_1d_read_empty_location_1_to_1::<F64>(DATABASE_LOCATION, &unpack_f64_stream);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _1d_f64_stream_read_location_50000_to_100000(b: &mut Bencher) {
    let test_fn = read_stream::_1d_read_empty_location_50000_to_100000::<F64>(DATABASE_LOCATION, &unpack_f64_stream);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _4d_f64_stream_read_location_1_1_1_1_to_1_1_1_1(b: &mut Bencher) {
    let test_fn = read_stream::_4d_read_empty_location_1_1_1_1_to_1_1_1_1::<F64>(DATABASE_LOCATION, &unpack_f64_stream);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _4d_f64_stream_read_location_1_1_1_50000_to_1_1_1_100000(b: &mut Bencher) {
    let test_fn = read_stream::_4d_read_empty_location_1_1_1_50000_to_1_1_1_100000::<F64>(DATABASE_LOCATION, &unpack_f64_stream);

    b.iter(|| {
        test_fn();
    });
}

fn unpack_f64_stream (stream: &mut Cursor<Vec<u8>>, n_values: usize, result: &mut [f64]) {
    unpack_stream(stream, n_values, |idx, value| {
        result[idx] = value
    }).unwrap()
}
