#![feature(test)]
mod constants;
extern crate test;

use std::io::{ Cursor, SeekFrom, Seek };
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
    let axis = [
        2,
    ];

    let n_values_to_read = 1usize;
    let point1 = [1];
    let value1 = Some(42.87);

    cql_db::create_db_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &axis
    ).unwrap();

    cql_db::write_value_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &point1,
        value1
    ).unwrap();

    let mut result: [Option<f64>; 1] = [None];
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream_unchecked::<NullableF64>(
            DATABASE_LOCATION,
            &mut stream,
            &point1,
            n_values_to_read as u64
        ).unwrap();

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, n_values_to_read, |idx, value| {
            result[idx] = value
        }).unwrap();
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
    let axis = [
        100000,
    ];

    let n_values_to_read = 50000usize;
    let base_point = [50000u64];
    let base_value = 42.87f64;

    cql_db::create_db_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &axis
    ).unwrap();

    for index in 0..n_values_to_read {
        cql_db::write_value_unchecked::<NullableF64>(
            DATABASE_LOCATION,
            &[base_point[0] + index as u64],
            Some(base_value + index as f64)
        ).unwrap();
    }

    let mut result: [Option<f64>; 50000] = [None; 50000];
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream_unchecked::<NullableF64>(
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
fn _4d_f64_nullable_stream_read_empty_location_1_1_1_1_to_1_1_1_1(b: &mut Bencher) {
    let axis = [
        2,
        2,
        2,
        2,
    ];
    let n_values_to_read = 1usize;
    let base_point = [1, 1, 1, 1];

    cql_db::create_db_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &axis
    ).unwrap();

    cql_db::link_dimensions_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &base_point[0..3],
    ).unwrap();

    let mut result: [Option<f64>; 1] = [None; 1];
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream_unchecked::<NullableF64>(
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
fn _4d_f64_nullable_stream_read_populated_location_1_1_1_1_to_1_1_1_1(b: &mut Bencher) {
    let axis = [
        2,
        2,
        2,
        2,
    ];

    let n_values_to_read = 1usize;
    let base_point = [1, 1, 1, 1];
    let base_value = Some(78352.3);

    cql_db::create_db_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &axis
    ).unwrap();

    cql_db::link_dimensions_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &base_point[0..3],
    ).unwrap();

    cql_db::write_value_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &base_point,
        base_value
    ).unwrap();

    let mut result: [Option<f64>; 1] = [None; 1];
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream_unchecked::<NullableF64>(
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
fn _4d_f64_nullable_stream_read_empty_location_1_1_1_50000_to_1_1_1_100000(b: &mut Bencher) {
    let axis = [
        2,
        2,
        2,
        100000,
    ];

    let n_values_to_read = 50000usize;
    let base_point = [1, 1, 1, 50000];

    cql_db::create_db_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &axis
    ).unwrap();

    cql_db::link_dimensions_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &base_point[0..3],
    ).unwrap();

    let mut result: [Option<f64>; 50000] = [None; 50000];
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream_unchecked::<NullableF64>(
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
fn _4d_f64_nullable_stream_read_populated_location_1_1_1_50000_to_1_1_1_100000(b: &mut Bencher) {
    let axis = [
        2,
        2,
        2,
        100000,
    ];

    let n_values_to_read = 50000usize;
    let base_point = [1, 1, 1, 50000];
    let base_value = 78352.3;

    cql_db::create_db_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &axis
    ).unwrap();

    cql_db::link_dimensions_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &base_point[0..3],
    ).unwrap();

    for index in 0..n_values_to_read {
        cql_db::write_value_unchecked::<NullableF64>(
            DATABASE_LOCATION,
            &[1, 1, 1, base_point[0] + index as u64],
            Some(base_value + index as f64)
        ).unwrap();
    }

    let mut result: [Option<f64>; 50000] = [None; 50000];
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream_unchecked::<NullableF64>(
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

fn unpack_nullable_f64_stream (stream: &mut Cursor<Vec<u8>>, n_values: usize, result: &mut [Option<f64>]) {
    unpack_stream(stream, n_values, |idx, value| {
        result[idx] = value
    }).unwrap()
}
