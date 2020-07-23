#![feature(test)]
mod constants;
extern crate test;

use std::io::{ Cursor, SeekFrom, Seek };
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
    const N_VALUES_TO_READ: usize = 50000;
    let point1 = [50000];
    let value1 = "1";

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &[100000]
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    ).unwrap();

    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream_unchecked::<TinyText>(
            DATABASE_LOCATION,
            &mut stream,
            &point1,
            N_VALUES_TO_READ as u64
        ).unwrap();

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, N_VALUES_TO_READ, |_, value| {
            result.push(value)
        }).unwrap();
    });
}

#[bench]
fn _1d_tiny_text_stream_read_255_char_location_50000_to_100000(b: &mut Bencher) {
    const N_VALUES_TO_READ: usize = 50000;
    let point1 = [50000];
    let value1 = "1".repeat(255);

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &[100000]
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    ).unwrap();

    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream_unchecked::<TinyText>(
            DATABASE_LOCATION,
            &mut stream,
            &point1,
            N_VALUES_TO_READ as u64
        ).unwrap();

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, N_VALUES_TO_READ, |_, value| {
            result.push(value)
        }).unwrap();
    });
}

#[bench]
fn _4d_tiny_text_stream_read_1_char_location_1_1_1_1_to_1_1_1_1(b: &mut Bencher) {
    const N_VALUES_TO_READ: usize = 1;
    let point1 = [1, 1, 1, 1];
    let value1 = "1".to_string();

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &[1, 1, 1, 1]
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1[0..3],
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    ).unwrap();

    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream_unchecked::<TinyText>(
            DATABASE_LOCATION,
            &mut stream,
            &point1,
            N_VALUES_TO_READ as u64
        ).unwrap();

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, N_VALUES_TO_READ, |_, value| {
            result.push(value)
        }).unwrap();
    });
}

#[bench]
fn _4d_tiny_text_stream_read_1_char_location_1_1_1_50000_to_1_1_1_100000(b: &mut Bencher) {
    const N_VALUES_TO_READ: usize = 50000;
    let point1 = [1, 1, 1, 50000];
    let value1 = "1";

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &[1, 1, 1, 100000]
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1[0..3],
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    ).unwrap();

    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream_unchecked::<TinyText>(
            DATABASE_LOCATION,
            &mut stream,
            &point1,
            N_VALUES_TO_READ as u64
        ).unwrap();

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, N_VALUES_TO_READ, |_, value| {
            result.push(value)
        }).unwrap();
    });
}

#[bench]
fn _4d_tiny_text_stream_read_255_char_location_1_1_1_50000_to_1_1_1_100000(b: &mut Bencher) {
    const N_VALUES_TO_READ: usize = 50000;
    let point1 = [1, 1, 1, 50000];
    let value1 = "1".repeat(255);

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &[1, 1, 1, 100000]
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1[0..3],
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    ).unwrap();

    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream_unchecked::<TinyText>(
            DATABASE_LOCATION,
            &mut stream,
            &point1,
            N_VALUES_TO_READ as u64
        ).unwrap();

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, N_VALUES_TO_READ, |_, value| {
            result.push(value)
        }).unwrap();
    });
}

fn unpack_tiny_text_stream (stream: &mut Cursor<Vec<u8>>, n_values: usize, result: &mut [TinyText]) {
    unpack_stream(stream, n_values, |idx, value| {
        result[idx] = value
    }).unwrap()
}
