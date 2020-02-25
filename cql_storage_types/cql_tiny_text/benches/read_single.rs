#![feature(test)]
mod constants;
extern crate test;

use constants::DATABASE_LOCATION;
use test::Bencher;
use std::convert::TryFrom;
use cql_tiny_text::TinyText;

#[bench]
fn _1d_tiny_text_single_point_read_0_char_location_1(b: &mut Bencher) {
    let point1 = [1];

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &[1]
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::new()
    ).unwrap();

    b.iter(|| {
        cql_db::read_value_unchecked::<TinyText>(
            DATABASE_LOCATION,
            &point1
        ).unwrap();
    });
}

#[bench]
fn _1d_tiny_text_single_point_read_1_char_location_1(b: &mut Bencher) {
    let point1 = [1];
    let value1 = "1";

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &[1]
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    ).unwrap();

    b.iter(|| {
        cql_db::read_value_unchecked::<TinyText>(
            DATABASE_LOCATION,
            &point1
        ).unwrap();
    });
}

#[bench]
fn _1d_tiny_text_single_point_read_255_char_location_1(b: &mut Bencher) {
    let point1 = [1];
    let value1 = "1".repeat(255);

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &[1]
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    ).unwrap();

    b.iter(|| {
        cql_db::read_value_unchecked::<TinyText>(
            DATABASE_LOCATION,
            &point1
        ).unwrap();
    });
}

#[bench]
fn _1d_tiny_text_single_point_read_1_char_location_100000(b: &mut Bencher) {
    let point1 = [100000];
    let value1 = "1".to_string();

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &[100000]
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    ).unwrap();

    b.iter(|| {
        cql_db::read_value_unchecked::<TinyText>(
            DATABASE_LOCATION,
            &point1
        ).unwrap();
    });
}

#[bench]
fn _4d_tiny_text_single_point_read_1_char_location_1_1_1_1(b: &mut Bencher) {
    let point1 = [1, 1, 1, 1];
    let value1 = "1";

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &[1, 1, 1, 1]
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1[0..3],
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    ).unwrap();

    b.iter(|| {
        cql_db::read_value_unchecked::<TinyText>(
            DATABASE_LOCATION,
            &point1
        ).unwrap();
    });
}

#[bench]
fn _4d_tiny_text_single_point_read_1_char_location_1_1_1_100000(b: &mut Bencher) {
    let point1 = [1, 1, 1, 100000];
    let value1 = "1";

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &[1, 1, 1, 100000]
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1[0..3],
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    ).unwrap();

    b.iter(|| {
        cql_db::read_value_unchecked::<TinyText>(
            DATABASE_LOCATION,
            &point1
        ).unwrap();
    });
}

#[bench]
fn _4d_tiny_text_single_point_read_255_char_location_1_1_1_100000(b: &mut Bencher) {
    let point1 = [1, 1, 1, 100000];
    let value1 = "1".repeat(255);

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &[1, 1, 1, 100000]
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1[0..3],
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    ).unwrap();

    b.iter(|| {
        cql_db::read_value_unchecked::<TinyText>(
            DATABASE_LOCATION,
            &point1
        ).unwrap();
    });
}

#[bench]
fn _4d_tiny_text_single_point_read_1_char_location_1_100000_1_1(b: &mut Bencher) {
    let point1 = [1, 100000, 1, 1];
    let value1 = "1";

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &[1, 100000, 1, 1]
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1[0..3],
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    ).unwrap();

    b.iter(|| {
        cql_db::read_value_unchecked::<TinyText>(
            DATABASE_LOCATION,
            &point1
        ).unwrap();
    });
}
