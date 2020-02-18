#![feature(test)]
mod constants;
extern crate test;

use std::io::{ Cursor, SeekFrom, Seek };
use constants::DATABASE_LOCATION;
use test::Bencher;
use std::convert::TryFrom;
use cql_tiny_text::{ TinyText, unpack_stream };

#[bench]
fn _1d_tiny_text_stream_read_0_char_location_1_to_1(b: &mut Bencher) {
    const N_VALUES_TO_READ: usize = 1;
    let point1 = [1];

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[1]
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::new()
    );

    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream::<TinyText>(
            DATABASE_LOCATION,
            &mut stream,
            &point1,
            N_VALUES_TO_READ as u64
        );

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, N_VALUES_TO_READ, |_, value| {
            result.push(value)
        });
    });
}

#[bench]
fn _1d_tiny_text_stream_read_1_char_location_1_to_1(b: &mut Bencher) {
    const N_VALUES_TO_READ: usize = 1;
    let point1 = [1];
    let value1 = "1";

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[1]
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    );

    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream::<TinyText>(
            DATABASE_LOCATION,
            &mut stream,
            &point1,
            N_VALUES_TO_READ as u64
        );

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, N_VALUES_TO_READ, |_, value| {
            result.push(value)
        });
    });
}

#[bench]
fn _1d_tiny_text_stream_read_255_char_location_1_to_1(b: &mut Bencher) {
    const N_VALUES_TO_READ: usize = 1;
    let point1 = [1];
    let value1 = "1".repeat(255);

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[1]
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    );

    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream::<TinyText>(
            DATABASE_LOCATION,
            &mut stream,
            &point1,
            N_VALUES_TO_READ as u64
        );

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, N_VALUES_TO_READ, |_, value| {
            result.push(value)
        });
    });
}

#[bench]
fn _1d_tiny_text_stream_read_1_char_location_50000_to_100000(b: &mut Bencher) {
    const N_VALUES_TO_READ: usize = 50000;
    let point1 = [50000];
    let value1 = "1";

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[100000]
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    );

    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream::<TinyText>(
            DATABASE_LOCATION,
            &mut stream,
            &point1,
            N_VALUES_TO_READ as u64
        );

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, N_VALUES_TO_READ, |_, value| {
            result.push(value)
        });
    });
}

#[bench]
fn _1d_tiny_text_stream_read_255_char_location_50000_to_100000(b: &mut Bencher) {
    const N_VALUES_TO_READ: usize = 50000;
    let point1 = [50000];
    let value1 = "1".repeat(255);

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[100000]
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    );

    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream::<TinyText>(
            DATABASE_LOCATION,
            &mut stream,
            &point1,
            N_VALUES_TO_READ as u64
        );

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, N_VALUES_TO_READ, |_, value| {
            result.push(value)
        });
    });
}

#[bench]
fn _4d_tiny_text_stream_read_1_char_location_1_1_1_1_to_1_1_1_1(b: &mut Bencher) {
    const N_VALUES_TO_READ: usize = 1;
    let point1 = [1, 1, 1, 1];
    let value1 = "1".to_string();

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[1, 1, 1, 1]
    );

    cql_db::link_dimensions::<TinyText>(
        DATABASE_LOCATION,
        &point1[0..3],
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    );

    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream::<TinyText>(
            DATABASE_LOCATION,
            &mut stream,
            &point1,
            N_VALUES_TO_READ as u64
        );

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, N_VALUES_TO_READ, |_, value| {
            result.push(value)
        });
    });
}

#[bench]
fn _4d_tiny_text_stream_read_1_char_location_1_1_1_50000_to_1_1_1_100000(b: &mut Bencher) {
    const N_VALUES_TO_READ: usize = 50000;
    let point1 = [1, 1, 1, 50000];
    let value1 = "1";

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[1, 1, 1, 100000]
    );

    cql_db::link_dimensions::<TinyText>(
        DATABASE_LOCATION,
        &point1[0..3],
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    );

    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream::<TinyText>(
            DATABASE_LOCATION,
            &mut stream,
            &point1,
            N_VALUES_TO_READ as u64
        );

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, N_VALUES_TO_READ, |_, value| {
            result.push(value)
        });
    });
}

#[bench]
fn _4d_tiny_text_stream_read_255_char_location_1_1_1_50000_to_1_1_1_100000(b: &mut Bencher) {
    const N_VALUES_TO_READ: usize = 50000;
    let point1 = [1, 1, 1, 50000];
    let value1 = "1".repeat(255);

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[1, 1, 1, 100000]
    );

    cql_db::link_dimensions::<TinyText>(
        DATABASE_LOCATION,
        &point1[0..3],
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    );

    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream::<TinyText>(
            DATABASE_LOCATION,
            &mut stream,
            &point1,
            N_VALUES_TO_READ as u64
        );

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, N_VALUES_TO_READ, |_, value| {
            result.push(value)
        });
    });
}
