#![feature(test)]
mod constants;
extern crate test;

use constants::DATABASE_LOCATION;
use test::{Bencher};

#[bench]
fn _1d_f64_nullable_single_point_read_location_1(b: &mut Bencher) {
    let axis = [
        cql_db::AxisDefinition {
            id: 1,
            max: 2,
        },
    ];

    let point1 = [1];
    let value1 = Some(42.87);

    cql_db::create_db::<Option<f64>>(
        DATABASE_LOCATION,
        &axis
    );

    cql_db::write_value::<Option<f64>>(
        DATABASE_LOCATION,
        &point1,
        value1
    );

    b.iter(|| {
        cql_db::read_value::<Option<f64>>(
            DATABASE_LOCATION,
            &point1
        );
    });
}

#[bench]
fn _1d_f64_nullable_single_point_read_location_100000(b: &mut Bencher) {
    let axis = [
        cql_db::AxisDefinition {
            id: 1,
            max: 100000,
        },
    ];

    let point1 = [100000];
    let value1 = Some(42.87);

    cql_db::create_db::<Option<f64>>(
        DATABASE_LOCATION,
        &axis
    );

    cql_db::write_value::<Option<f64>>(
        DATABASE_LOCATION,
        &point1,
        value1
    );

    b.iter(|| {
        cql_db::read_value::<Option<f64>>(
            DATABASE_LOCATION,
            &point1
        );
    });
}

#[bench]
fn _4d_f64_nullable_single_point_read_location_1_1_1_1(b: &mut Bencher) {
    let axis = [
        cql_db::AxisDefinition {
            id: 1,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 2,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 3,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 4,
            max: 2
        },
    ];

    let point1 = [1, 1, 1, 1];
    let value1 = Some(-5.6);

    cql_db::create_db::<Option<f64>>(
        DATABASE_LOCATION,
        &axis
    );

    let first_to_second_key = cql_db::add_key::<Option<f64>>(
        DATABASE_LOCATION,
        point1[0],
        point1[1],
        &axis[0],
        &axis[1]
    );

    cql_db::add_key::<Option<f64>>(
        DATABASE_LOCATION,
        first_to_second_key,
        point1[2],
        &axis[1],
        &axis[2]
    );

    cql_db::write_value::<Option<f64>>(
        DATABASE_LOCATION,
        &point1,
        value1
    );

    b.iter(|| {
        cql_db::read_value::<Option<f64>>(
            DATABASE_LOCATION,
            &point1
        );
    });
}

#[bench]
fn _4d_f64_nullable_single_point_read_location_1_1_1_100000(b: &mut Bencher) {
    let axis = [
        cql_db::AxisDefinition {
            id: 1,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 2,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 3,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 4,
            max: 100000
        },
    ];

    let point1 = [1, 1, 1, 100000];
    let value1 = Some(-5.6);

    cql_db::create_db::<Option<f64>>(
        DATABASE_LOCATION,
        &axis
    );

    let first_to_second_key = cql_db::add_key::<Option<f64>>(
        DATABASE_LOCATION,
        point1[0],
        point1[1],
        &axis[0],
        &axis[1]
    );

    cql_db::add_key::<Option<f64>>(
        DATABASE_LOCATION,
        first_to_second_key,
        point1[2],
        &axis[1],
        &axis[2]
    );

    cql_db::write_value::<Option<f64>>(
        DATABASE_LOCATION,
        &point1,
        value1
    );

    b.iter(|| {
        cql_db::read_value::<Option<f64>>(
            DATABASE_LOCATION,
            &point1
        );
    });
}

#[bench]
fn _4d_f64_nullable_single_point_read_location_1_100000_1_1(b: &mut Bencher) {
    let axis = [
        cql_db::AxisDefinition {
            id: 1,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 2,
            max: 100000,
        },
        cql_db::AxisDefinition {
            id: 3,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 4,
            max: 2
        },
    ];

    let point1 = [1, 100000, 1, 1];
    let value1 = Some(-5.6);

    cql_db::create_db::<Option<f64>>(
        DATABASE_LOCATION,
        &axis
    );

    let first_to_second_key = cql_db::add_key::<Option<f64>>(
        DATABASE_LOCATION,
        point1[0],
        point1[1],
        &axis[0],
        &axis[1]
    );

    cql_db::add_key::<Option<f64>>(
        DATABASE_LOCATION,
        first_to_second_key,
        point1[2],
        &axis[1],
        &axis[2]
    );

    cql_db::write_value::<Option<f64>>(
        DATABASE_LOCATION,
        &point1,
        value1
    );

    b.iter(|| {
        cql_db::read_value::<Option<f64>>(
            DATABASE_LOCATION,
            &point1
        );
    });
}
