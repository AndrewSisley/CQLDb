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
    let point1 = [1, 1, 1, 1];
    let value1 = Some(5.4);

    cql_db::create_db_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &[1, 1, 1, 1]
    ).unwrap();

    cql_db::link_dimensions_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &point1[0..3],
    ).unwrap();

    b.iter(|| {
        cql_db::write_value_unchecked::<NullableF64>(
            DATABASE_LOCATION,
            &point1,
            value1
        ).unwrap();
    });
}

#[bench]
fn _4d_f64_nullable_single_point_write_location_1_1_1_100000(b: &mut Bencher) {
    let point1 = [1, 1, 1, 100000];
    let value1 = Some(5.6);

    cql_db::create_db_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &[1, 1, 1, 100000]
    ).unwrap();

    cql_db::link_dimensions_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &point1[0..3],
    ).unwrap();

    b.iter(|| {
        cql_db::write_value_unchecked::<NullableF64>(
            DATABASE_LOCATION,
            &point1,
            value1
        ).unwrap();
    });
}

#[bench]
fn _4d_f64_nullable_single_point_write_location_1_100000_1_1(b: &mut Bencher) {
    let point1 = [1, 100000, 1, 1];
    let value1 = Some(5.5);

    cql_db::create_db_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &[1, 100000, 1, 1]
    ).unwrap();

    cql_db::link_dimensions_unchecked::<NullableF64>(
        DATABASE_LOCATION,
        &point1[0..3],
    ).unwrap();

    b.iter(|| {
        cql_db::write_value_unchecked::<NullableF64>(
            DATABASE_LOCATION,
            &point1,
            value1
        ).unwrap();
    });
}
