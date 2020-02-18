#![feature(test)]
mod constants;
extern crate test;

use constants::DATABASE_LOCATION;
use test::Bencher;
use std::convert::TryFrom;
use cql_tiny_text::TinyText;

#[bench]
fn _1d_tiny_text_single_point_write_empty_location_1(b: &mut Bencher) {
    let point1 = [1];

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[1]
    );

    b.iter(|| {
        cql_db::write_value::<TinyText>(
            DATABASE_LOCATION,
            &point1,
            TinyText::new()
        );
    });
}

#[bench]
fn _1d_tiny_text_single_point_write_1_char_location_1(b: &mut Bencher) {
    let point1 = [1];
    let value1 = "1";

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[1]
    );

    b.iter(|| {
        cql_db::write_value::<TinyText>(
            DATABASE_LOCATION,
            &point1,
            TinyText::try_from(value1).unwrap()
        );
    });
}

#[bench]
fn _1d_tiny_text_single_point_write_255_char_location_1(b: &mut Bencher) {
    let point1 = [1];
    let value1 = "1";

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[1]
    );

    b.iter(|| {
        cql_db::write_value::<TinyText>(
            DATABASE_LOCATION,
            &point1,
            TinyText::try_from(value1.repeat(255)).unwrap()
        );
    });
}

#[bench]
fn _1d_tiny_text_single_point_write_1_char_location_100000(b: &mut Bencher) {
    let point1 = [100000];
    let value1 = "1";

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[100000]
    );

    b.iter(|| {
        cql_db::write_value::<TinyText>(
            DATABASE_LOCATION,
            &point1,
            TinyText::try_from(value1).unwrap()
        );
    });
}

#[bench]
fn _4d_tiny_text_single_point_write_1_char_location_1_1_1_1(b: &mut Bencher) {
    let point1 = [1, 1, 1, 1];
    let value1 = "1";

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[1, 1, 1, 1]
    );

    cql_db::link_dimensions::<TinyText>(
        DATABASE_LOCATION,
        &point1[0..3],
    );

    b.iter(|| {
        cql_db::write_value::<TinyText>(
            DATABASE_LOCATION,
            &point1,
            TinyText::try_from(value1).unwrap()
        );
    });
}

#[bench]
fn _4d_tiny_text_single_point_write_1_char_location_1_1_1_100000(b: &mut Bencher) {
    let point1 = [1, 1, 1, 100000];
    let value1 = "1";

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[1, 1, 1, 100000]
    );

    cql_db::link_dimensions::<TinyText>(
        DATABASE_LOCATION,
        &point1[0..3],
    );

    b.iter(|| {
        cql_db::write_value::<TinyText>(
            DATABASE_LOCATION,
            &point1,
            TinyText::try_from(value1).unwrap()
        );
    });
}

#[bench]
fn _4d_tiny_text_single_point_write_255_char_location_1_1_1_100000(b: &mut Bencher) {
    let point1 = [1, 1, 1, 100000];
    let value1 = "1";

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[1, 1, 1, 100000]
    );

    cql_db::link_dimensions::<TinyText>(
        DATABASE_LOCATION,
        &point1[0..3],
    );

    b.iter(|| {
        cql_db::write_value::<TinyText>(
            DATABASE_LOCATION,
            &point1,
            TinyText::try_from(value1.repeat(255)).unwrap()
        );
    });
}

#[bench]
fn _4d_tiny_text_single_point_write_1_char_location_1_100000_1_1(b: &mut Bencher) {
    let point1 = [1, 100000, 1, 1];
    let value1 = "1";

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &[1, 100000, 1, 1]
    );

    cql_db::link_dimensions::<TinyText>(
        DATABASE_LOCATION,
        &point1[0..3],
    );

    b.iter(|| {
        cql_db::write_value::<TinyText>(
            DATABASE_LOCATION,
            &point1,
            TinyText::try_from(value1).unwrap()
        );
    });
}
