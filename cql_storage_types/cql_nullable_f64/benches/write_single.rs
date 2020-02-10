#![feature(test)]
mod constants;
extern crate test;

use constants::DATABASE_LOCATION;
use test::Bencher;
use cql_nullable_f64::NullableF64;

#[bench]
fn _1d_f64_nullable_single_point_write_location_1(b: &mut Bencher) {
    let point1 = [1];
    let value1 = Some(42.1);

    cql_db::create_db::<NullableF64>(
        DATABASE_LOCATION,
        &[1]
    );

    b.iter(|| {
        cql_db::write_value::<NullableF64>(
            DATABASE_LOCATION,
            &point1,
            value1
        );
    });
}

#[bench]
fn _1d_f64_nullable_single_point_write_location_100000(b: &mut Bencher) {
    let point1 = [100000];
    let value1 = Some(42.1);

    cql_db::create_db::<NullableF64>(
        DATABASE_LOCATION,
        &[100000]
    );

    b.iter(|| {
        cql_db::write_value::<NullableF64>(
            DATABASE_LOCATION,
            &point1,
            value1
        );
    });
}

#[bench]
fn _4d_f64_nullable_single_point_write_location_1_1_1_1(b: &mut Bencher) {
    let point1 = [1, 1, 1, 1];
    let value1 = Some(5.4);

    cql_db::create_db::<NullableF64>(
        DATABASE_LOCATION,
        &[1, 1, 1, 1]
    );

    cql_db::link_dimensions::<NullableF64>(
        DATABASE_LOCATION,
        &point1[0..3],
    );

    b.iter(|| {
        cql_db::write_value::<NullableF64>(
            DATABASE_LOCATION,
            &point1,
            value1
        );
    });
}

#[bench]
fn _4d_f64_nullable_single_point_write_location_1_1_1_100000(b: &mut Bencher) {
    let point1 = [1, 1, 1, 100000];
    let value1 = Some(5.6);

    cql_db::create_db::<NullableF64>(
        DATABASE_LOCATION,
        &[1, 1, 1, 100000]
    );

    cql_db::link_dimensions::<NullableF64>(
        DATABASE_LOCATION,
        &point1[0..3],
    );

    b.iter(|| {
        cql_db::write_value::<NullableF64>(
            DATABASE_LOCATION,
            &point1,
            value1
        );
    });
}

#[bench]
fn _4d_f64_nullable_single_point_write_location_1_100000_1_1(b: &mut Bencher) {
    let point1 = [1, 100000, 1, 1];
    let value1 = Some(5.5);

    cql_db::create_db::<NullableF64>(
        DATABASE_LOCATION,
        &[1, 100000, 1, 1]
    );

    cql_db::link_dimensions::<NullableF64>(
        DATABASE_LOCATION,
        &point1[0..3],
    );

    b.iter(|| {
        cql_db::write_value::<NullableF64>(
            DATABASE_LOCATION,
            &point1,
            value1
        );
    });
}
