#![feature(test)]
mod constants;
extern crate test;

use std::io::{ Cursor, SeekFrom, Seek };
use constants::DATABASE_LOCATION;
use test::{ Bencher };
use cql_i16::{ unpack_stream, I16 };
use cql_storage_type_testing_lib::benches::read_stream;

#[bench]
fn _1d_i16_stream_read_location_1_to_1(b: &mut Bencher) {
    let test_fn = read_stream::_1d_read_empty_location_1_to_1::<I16>(DATABASE_LOCATION, &unpack_i16_stream);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _1d_i16_stream_read_location_50000_to_100000(b: &mut Bencher) {
    let n_values_to_read = 50000usize;
    let base_point = [50000u64];

    cql_db::create_db_unchecked::<I16>(
        DATABASE_LOCATION,
        &[100000]
    ).unwrap();

    let mut result = [0; 50000];
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream_unchecked::<I16>(
            DATABASE_LOCATION,
            &mut stream,
            &base_point,
            n_values_to_read as u64
        ).unwrap();

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, n_values_to_read, |idx, value| {
            result[idx] = value
        }).unwrap();
    });
}

#[bench]
fn _4d_i16_stream_read_location_1_1_1_1_to_1_1_1_1(b: &mut Bencher) {
    let n_values_to_read = 1usize;
    let base_point = [1, 1, 1, 1];

    cql_db::create_db_unchecked::<I16>(
        DATABASE_LOCATION,
        &[1, 1, 1, 1]
    ).unwrap();

    cql_db::link_dimensions_unchecked::<I16>(
        DATABASE_LOCATION,
        &base_point[0..3],
    ).unwrap();

    let mut result = [0];
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream_unchecked::<I16>(
            DATABASE_LOCATION,
            &mut stream,
            &base_point,
            n_values_to_read as u64
        ).unwrap();

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, n_values_to_read, |idx, value| {
            result[idx] = value
        }).unwrap();
    });
}

#[bench]
fn _4d_i16_stream_read_location_1_1_1_50000_to_1_1_1_100000(b: &mut Bencher) {
    let n_values_to_read = 50000usize;
    let base_point = [1, 1, 1, 50000];

    cql_db::create_db_unchecked::<I16>(
        DATABASE_LOCATION,
        &[1, 1, 1, 100000]
    ).unwrap();

    cql_db::link_dimensions_unchecked::<I16>(
        DATABASE_LOCATION,
        &base_point[0..3],
    ).unwrap();

    let mut result = [0; 50000];
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream_unchecked::<I16>(
            DATABASE_LOCATION,
            &mut stream,
            &base_point,
            n_values_to_read as u64
        ).unwrap();

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, n_values_to_read, |idx, value| {
            result[idx] = value
        }).unwrap();
    });
}

fn unpack_i16_stream (stream: &mut Cursor<Vec<u8>>, n_values: usize, result: &mut [i16]) {
    unpack_stream(stream, n_values, |idx, value| {
        result[idx] = value
    }).unwrap()
}
