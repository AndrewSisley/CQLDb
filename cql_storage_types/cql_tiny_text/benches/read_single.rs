#![feature(test)]
mod constants;
extern crate test;

use constants::DATABASE_LOCATION;
use test::Bencher;
use std::convert::TryFrom;
use cql_tiny_text::TinyText;
use cql_storage_type_testing_lib::benches::read_single;

#[bench]
fn _1d_tiny_text_single_point_read_0_char_location_1(b: &mut Bencher) {
    let test_fn = read_single::_1d_read_location_1::<TinyText>(DATABASE_LOCATION, TinyText::new());

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _1d_tiny_text_single_point_read_1_char_location_1(b: &mut Bencher) {
    let value1 = "1";
    let test_fn = read_single::_1d_read_location_1::<TinyText>(DATABASE_LOCATION, TinyText::try_from(value1).unwrap());

    b.iter(|| {
        test_fn();
    });
}

#[bench]
fn _1d_tiny_text_single_point_read_255_char_location_1(b: &mut Bencher) {
    let value1 = "1".repeat(255);
    let test_fn = read_single::_1d_read_location_1::<TinyText>(DATABASE_LOCATION, TinyText::try_from(value1).unwrap());

    b.iter(|| {
        test_fn();
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
