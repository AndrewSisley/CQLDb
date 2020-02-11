#![feature(test)]
mod constants;
extern crate test;

use constants::DATABASE_LOCATION;
use test::Bencher;
use cql_tiny_text::TinyText;

#[bench]
fn _1d_tiny_text_single_point_read_0_char_location_1(b: &mut Bencher) {
    let point1 = [1];
    let value1 = String::new();

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[1]
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        value1
    );

    b.iter(|| {
        cql_db::read_value::<TinyText>(
            DATABASE_LOCATION,
            &point1
        );
    });
}

#[bench]
fn _1d_tiny_text_single_point_read_1_char_location_1(b: &mut Bencher) {
    let point1 = [1];
    let value1 = "1".to_string();

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[1]
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        value1
    );

    b.iter(|| {
        cql_db::read_value::<TinyText>(
            DATABASE_LOCATION,
            &point1
        );
    });
}

#[bench]
fn _1d_tiny_text_single_point_read_255_char_location_1(b: &mut Bencher) {
    let point1 = [1];
    let value1 = "1".repeat(255);

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[1]
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        value1
    );

    b.iter(|| {
        cql_db::read_value::<TinyText>(
            DATABASE_LOCATION,
            &point1
        );
    });
}

#[bench]
fn _1d_tiny_text_single_point_read_1_char_location_100000(b: &mut Bencher) {
    let point1 = [100000];
    let value1 = "1".to_string();

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[100000]
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        value1
    );

    b.iter(|| {
        cql_db::read_value::<TinyText>(
            DATABASE_LOCATION,
            &point1
        );
    });
}

#[bench]
fn _4d_tiny_text_single_point_read_1_char_location_1_1_1_1(b: &mut Bencher) {
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
        value1
    );

    b.iter(|| {
        cql_db::read_value::<TinyText>(
            DATABASE_LOCATION,
            &point1
        );
    });
}

#[bench]
fn _4d_tiny_text_single_point_read_1_char_location_1_1_1_100000(b: &mut Bencher) {
    let point1 = [1, 1, 1, 100000];
    let value1 = "1".to_string();

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
        value1
    );

    b.iter(|| {
        cql_db::read_value::<TinyText>(
            DATABASE_LOCATION,
            &point1
        );
    });
}

#[bench]
fn _4d_tiny_text_single_point_read_255_char_location_1_1_1_100000(b: &mut Bencher) {
    let point1 = [1, 1, 1, 100000];
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
        value1
    );

    b.iter(|| {
        cql_db::read_value::<TinyText>(
            DATABASE_LOCATION,
            &point1
        );
    });
}

#[bench]
fn _4d_tiny_text_single_point_read_1_char_location_1_100000_1_1(b: &mut Bencher) {
    let point1 = [1, 100000, 1, 1];
    let value1 = "1".to_string();

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[1, 100000, 1, 1]
    );

    cql_db::link_dimensions::<TinyText>(
        DATABASE_LOCATION,
        &point1[0..3],
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        value1
    );

    b.iter(|| {
        cql_db::read_value::<TinyText>(
            DATABASE_LOCATION,
            &point1
        );
    });
}
