#![feature(test)]
mod constants;
extern crate test;

use constants::DATABASE_LOCATION;
use test::Bencher;
use cql_nullable_f64::NullableF64;
use cql_storage_type_testing_lib::benches::write_single;

#[bench]
fn _1d_f64_nullable_single_point_write_location_1(b: &mut Bencher) {
    let test_fn = write_single::_1d_write_location_1::<NullableF64>(DATABASE_LOCATION);

    b.iter(|| {
        test_fn(Some(42.1));
    });
}

#[bench]
fn _1d_f64_nullable_single_point_write_location_100000(b: &mut Bencher) {
    let test_fn = write_single::_1d_write_location_100000::<NullableF64>(DATABASE_LOCATION);

    b.iter(|| {
        test_fn(Some(42.1));
    });
}

#[bench]
fn _4d_f64_nullable_single_point_write_location_1_1_1_1(b: &mut Bencher) {
    let test_fn = write_single::_4d_write_location_1_1_1_1::<NullableF64>(DATABASE_LOCATION);

    b.iter(|| {
        test_fn(Some(5.4));
    });
}

#[bench]
fn _4d_f64_nullable_single_point_write_location_1_1_1_100000(b: &mut Bencher) {
    let test_fn = write_single::_4d_write_location_1_1_1_100000::<NullableF64>(DATABASE_LOCATION);

    b.iter(|| {
        test_fn(Some(5.6));
    });
}

#[bench]
fn _4d_f64_nullable_single_point_write_location_1_100000_1_1(b: &mut Bencher) {
    let test_fn = write_single::_4d_write_location_1_100000_1_1::<NullableF64>(DATABASE_LOCATION);

    b.iter(|| {
        test_fn(Some(5.5));
    });
}
