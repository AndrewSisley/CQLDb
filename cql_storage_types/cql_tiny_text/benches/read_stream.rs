#![feature(test)]
mod constants;
extern crate test;

use std::io::{ Cursor };
use constants::DATABASE_LOCATION;
use test::Bencher;
use std::convert::TryFrom;
use cql_tiny_text::{ TinyText, unpack_stream };
use cql_storage_type_testing_lib::benches::read_stream;

#[bench]
fn _1d_tiny_text_stream_read_0_char_location_1_to_1(b: &mut Bencher) {
    let test_fn = read_stream::_1d_read_empty_location_1_to_1::<TinyText>(DATABASE_LOCATION, &unpack_tiny_text_stream);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _1d_tiny_text_stream_read_1_char_location_1_to_1(b: &mut Bencher) {
    let value1 = "1";
    let test_fn = read_stream::_1d_read_populated_location_1_to_1::<TinyText>(DATABASE_LOCATION, TinyText::try_from(value1).unwrap(), &unpack_tiny_text_stream);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _1d_tiny_text_stream_read_255_char_location_1_to_1(b: &mut Bencher) {
    let value1 = "1".repeat(255);
    let test_fn = read_stream::_1d_read_populated_location_1_to_1::<TinyText>(DATABASE_LOCATION, TinyText::try_from(value1).unwrap(), &unpack_tiny_text_stream);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _1d_tiny_text_stream_read_1_char_location_50000_to_100000(b: &mut Bencher) {
    let test_fn = read_stream::_1d_read_populated_location_50000_to_100000::<TinyText>(
        DATABASE_LOCATION,
        &|_| TinyText::try_from("1").unwrap(),
        &unpack_tiny_text_stream
    );

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _1d_tiny_text_stream_read_255_char_location_50000_to_100000(b: &mut Bencher) {
    let test_fn = read_stream::_1d_read_populated_location_50000_to_100000::<TinyText>(
        DATABASE_LOCATION,
        &|_| TinyText::try_from("1".repeat(255)).unwrap(),
        &unpack_tiny_text_stream
    );

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _4d_tiny_text_stream_read_1_char_location_1_1_1_1_to_1_1_1_1(b: &mut Bencher) {
    let value1 = "1";
    let test_fn = read_stream::_4d_read_populated_location_1_1_1_1_to_1_1_1_1::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from(value1).unwrap(),
        &unpack_tiny_text_stream
    );

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _4d_tiny_text_stream_read_1_char_location_1_1_1_50000_to_1_1_1_100000(b: &mut Bencher) {
    let test_fn = read_stream::_4d_read_populated_location_1_1_1_50000_to_1_1_1_100000::<TinyText>(
        DATABASE_LOCATION,
        &|_| TinyText::try_from("1").unwrap(),
        &unpack_tiny_text_stream
    );

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _4d_tiny_text_stream_read_255_char_location_1_1_1_50000_to_1_1_1_100000(b: &mut Bencher) {
    let test_fn = read_stream::_4d_read_populated_location_1_1_1_50000_to_1_1_1_100000::<TinyText>(
        DATABASE_LOCATION,
        &|_| TinyText::try_from("1".repeat(255)).unwrap(),
        &unpack_tiny_text_stream
    );

    b.iter(|| {
        test_fn();
    });
}

fn unpack_tiny_text_stream (stream: &mut Cursor<Vec<u8>>, n_values: usize, result: &mut [TinyText]) {
    unpack_stream(stream, n_values, |idx, value| {
        result[idx] = value
    }).unwrap()
}
