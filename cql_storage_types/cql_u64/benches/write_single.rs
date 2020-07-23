#![feature(test)]
mod constants;
extern crate test;

use constants::DATABASE_LOCATION;
use test::Bencher;
use cql_u64::U64;
use cql_storage_type_testing_lib::benches::write_single;

#[bench]
fn _1d_u64_single_point_write_location_1(b: &mut Bencher) {
    let test_fn = write_single::_1d_write_location_1::<U64>(DATABASE_LOCATION);

    b.iter(|| {
        test_fn(42);
    });
}

#[bench]
fn _1d_u64_single_point_write_location_100000(b: &mut Bencher) {
    let axis = [
        100000,
    ];

    let point1 = [100000];
    let value1 = 42;

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &axis
    ).unwrap();

    b.iter(|| {
        cql_db::write_value_unchecked::<U64>(
            DATABASE_LOCATION,
            &point1,
            value1
        ).unwrap();
    });
}

#[bench]
fn _4d_u64_single_point_write_location_1_1_1_1(b: &mut Bencher) {
    let axis = [
        2,
        2,
        2,
        2,
    ];

    let point1 = [1, 1, 1, 1];
    let value1 = 5;

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &axis
    ).unwrap();

    cql_db::link_dimensions_unchecked::<U64>(
        DATABASE_LOCATION,
        &point1[0..3],
    ).unwrap();

    b.iter(|| {
        cql_db::write_value_unchecked::<U64>(
            DATABASE_LOCATION,
            &point1,
            value1
        ).unwrap();
    });
}

#[bench]
fn _4d_u64_single_point_write_location_1_1_1_100000(b: &mut Bencher) {
    let axis = [
        2,
        2,
        2,
        100000,
    ];

    let point1 = [1, 1, 1, 100000];
    let value1 = 5;

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &axis
    ).unwrap();

    cql_db::link_dimensions_unchecked::<U64>(
        DATABASE_LOCATION,
        &point1[0..3],
    ).unwrap();

    b.iter(|| {
        cql_db::write_value_unchecked::<U64>(
            DATABASE_LOCATION,
            &point1,
            value1
        ).unwrap();
    });
}

#[bench]
fn _4d_u64_single_point_write_location_1_100000_1_1(b: &mut Bencher) {
    let axis = [
        2,
        100000,
        2,
        2,
    ];

    let point1 = [1, 100000, 1, 1];
    let value1 = 5;

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &axis
    ).unwrap();

    cql_db::link_dimensions_unchecked::<U64>(
        DATABASE_LOCATION,
        &point1[0..3],
    ).unwrap();

    b.iter(|| {
        cql_db::write_value_unchecked::<U64>(
            DATABASE_LOCATION,
            &point1,
            value1
        ).unwrap();
    });
}
