#![feature(test)]
mod constants;
extern crate test;

use constants::DATABASE_LOCATION;
use test::Bencher;
use cql_u64::U64;
use cql_storage_type_testing_lib::benches::read_single;

#[bench]
fn _1d_u64_single_point_read_location_1(b: &mut Bencher) {
    let test_fn = read_single::_1d_read_location_1::<U64>(DATABASE_LOCATION, 42);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _1d_u64_single_point_read_location_100000(b: &mut Bencher) {
    let test_fn = read_single::_1d_read_location_100000::<U64>(DATABASE_LOCATION, 42);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _4d_u64_single_point_read_location_1_1_1_1(b: &mut Bencher) {
    let test_fn = read_single::_4d_read_location_1_1_1_1::<U64>(DATABASE_LOCATION, 5);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _4d_u64_single_point_read_location_1_1_1_100000(b: &mut Bencher) {
    let test_fn = read_single::_4d_read_location_1_1_1_100000::<U64>(DATABASE_LOCATION, 5);

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _4d_u64_single_point_read_location_1_100000_1_1(b: &mut Bencher) {
    let test_fn = read_single::_4d_read_location_1_100000_1_1::<U64>(DATABASE_LOCATION, 5);

    b.iter(|| {
        test_fn();
    });
}
