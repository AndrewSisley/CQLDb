#![feature(test)]
mod constants;
extern crate test;

use constants::DATABASE_LOCATION;
use test::Bencher;
use cql_i16::I16;
use cql_storage_type_testing_lib::benches::write_single;

#[bench]
fn _1d_i16_single_point_write_location_1(b: &mut Bencher) {
    let test_fn = write_single::_1d_write_location_1::<I16>(DATABASE_LOCATION);

    b.iter(|| {
        test_fn(42);
    });
}

#[bench]
fn _1d_i16_single_point_write_location_100000(b: &mut Bencher) {
    let test_fn = write_single::_1d_write_location_100000::<I16>(DATABASE_LOCATION);

    b.iter(|| {
        test_fn(42);
    });
}

#[bench]
fn _4d_i16_single_point_write_location_1_1_1_1(b: &mut Bencher) {
    let test_fn = write_single::_4d_write_location_1_1_1_1::<I16>(DATABASE_LOCATION);

    b.iter(|| {
        test_fn(5);
    });
}

#[bench]
fn _4d_i16_single_point_write_location_1_1_1_100000(b: &mut Bencher) {
    let test_fn = write_single::_4d_write_location_1_1_1_100000::<I16>(DATABASE_LOCATION);

    b.iter(|| {
        test_fn(5);
    });
}

#[bench]
fn _4d_i16_single_point_write_location_1_100000_1_1(b: &mut Bencher) {
    let test_fn = write_single::_4d_write_location_1_100000_1_1::<I16>(DATABASE_LOCATION);

    b.iter(|| {
        test_fn(5);
    });
}
