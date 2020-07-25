#![feature(test)]
mod constants;
extern crate test;

use std::io::{ Cursor };
use constants::DATABASE_LOCATION;
use test::{ Bencher };
use cql_nullable_f64::{ unpack_stream, NullableF64 };
use cql_storage_type_testing_lib::benches::read_stream;

#[bench]
fn _1d_f64_nullable_stream_read_empty_location_1_to_1(b: &mut Bencher) {
    let test_fn = read_stream::_1d_read_empty_location_1_to_1::<NullableF64>(DATABASE_LOCATION, &unpack_nullable_f64_stream);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _1d_f64_nullable_stream_read_populated_location_1_to_1(b: &mut Bencher) {
    let test_fn = read_stream::_1d_read_populated_location_1_to_1::<NullableF64>(DATABASE_LOCATION, Some(42.87), &unpack_nullable_f64_stream);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _1d_f64_nullable_stream_read_empty_location_50000_to_100000(b: &mut Bencher) {
    let test_fn = read_stream::_1d_read_empty_location_50000_to_100000::<NullableF64>(DATABASE_LOCATION, &unpack_nullable_f64_stream);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _1d_f64_nullable_stream_read_populated_location_50000_to_100000(b: &mut Bencher) {
    let test_fn = read_stream::_1d_read_populated_location_50000_to_100000::<NullableF64>(DATABASE_LOCATION, &|_| Some(42.87), &unpack_nullable_f64_stream);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _4d_f64_nullable_stream_read_empty_location_1_1_1_1_to_1_1_1_1(b: &mut Bencher) {
    let test_fn = read_stream::_4d_read_empty_location_1_1_1_1_to_1_1_1_1::<NullableF64>(DATABASE_LOCATION, &unpack_nullable_f64_stream);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _4d_f64_nullable_stream_read_populated_location_1_1_1_1_to_1_1_1_1(b: &mut Bencher) {
    let test_fn = read_stream::_4d_read_populated_location_1_1_1_1_to_1_1_1_1::<NullableF64>(DATABASE_LOCATION, Some(78352.3), &unpack_nullable_f64_stream);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _4d_f64_nullable_stream_read_empty_location_1_1_1_50000_to_1_1_1_100000(b: &mut Bencher) {
    let test_fn = read_stream::_4d_read_empty_location_1_1_1_50000_to_1_1_1_100000::<NullableF64>(DATABASE_LOCATION, &unpack_nullable_f64_stream);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _4d_f64_nullable_stream_read_populated_location_1_1_1_50000_to_1_1_1_100000(b: &mut Bencher) {
    let test_fn = read_stream::_4d_read_populated_location_1_1_1_50000_to_1_1_1_100000::<NullableF64>(DATABASE_LOCATION, &|_| Some(78352.3), &unpack_nullable_f64_stream);

    b.iter(|| {
        test_fn();
    });
}

fn unpack_nullable_f64_stream (stream: &mut Cursor<Vec<u8>>, n_values: usize, result: &mut [Option<f64>]) {
    unpack_stream(stream, n_values, |idx, value| {
        result[idx] = value
    }).unwrap()
}
