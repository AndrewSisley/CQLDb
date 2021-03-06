#![feature(test)]
mod constants;
extern crate test;

use constants::DATABASE_LOCATION;
use test::Bencher;
use std::convert::TryFrom;
use cql_tiny_text::TinyText;
use cql_storage_type_testing_lib::benches::write_single;

#[bench]
fn _1d_tiny_text_single_point_write_empty_location_1(b: &mut Bencher) {
    let test_fn = write_single::_1d_write_location_1::<TinyText>(DATABASE_LOCATION);

    b.iter(|| {
        test_fn(TinyText::new());
    });
}

#[bench]
fn _1d_tiny_text_single_point_write_1_char_location_1(b: &mut Bencher) {
    let value1 = "1";
    let test_fn = write_single::_1d_write_location_1::<TinyText>(DATABASE_LOCATION);

    b.iter(|| {
        test_fn(TinyText::try_from(value1).unwrap());
    });
}

#[bench]
fn _1d_tiny_text_single_point_write_255_char_location_1(b: &mut Bencher) {
    let value1 = "1";
    let test_fn = write_single::_1d_write_location_1::<TinyText>(DATABASE_LOCATION);

    b.iter(|| {
        test_fn(TinyText::try_from(value1.repeat(255)).unwrap());
    });
}

#[bench]
fn _1d_tiny_text_single_point_write_1_char_location_100000(b: &mut Bencher) {
    let value1 = "1";
    let test_fn = write_single::_1d_write_location_100000::<TinyText>(DATABASE_LOCATION);

    b.iter(|| {
        test_fn(TinyText::try_from(value1).unwrap());
    });
}

#[bench]
fn _4d_tiny_text_single_point_write_1_char_location_1_1_1_1(b: &mut Bencher) {
    let value1 = "1";
    let test_fn = write_single::_4d_write_location_1_1_1_1::<TinyText>(DATABASE_LOCATION);

    b.iter(|| {
        test_fn(TinyText::try_from(value1).unwrap());
    });
}

#[bench]
fn _4d_tiny_text_single_point_write_1_char_location_1_1_1_100000(b: &mut Bencher) {
    let value1 = "1";
    let test_fn = write_single::_4d_write_location_1_1_1_100000::<TinyText>(DATABASE_LOCATION);

    b.iter(|| {
        test_fn(TinyText::try_from(value1).unwrap());
    });
}

#[bench]
fn _4d_tiny_text_single_point_write_255_char_location_1_1_1_100000(b: &mut Bencher) {
    let value1 = "1";
    let test_fn = write_single::_4d_write_location_1_1_1_100000::<TinyText>(DATABASE_LOCATION);

    b.iter(|| {
        test_fn(TinyText::try_from(value1.repeat(255)).unwrap());
    });
}

#[bench]
fn _4d_tiny_text_single_point_write_1_char_location_1_100000_1_1(b: &mut Bencher) {
    let value1 = "1";
    let test_fn = write_single::_4d_write_location_1_100000_1_1::<TinyText>(DATABASE_LOCATION);

    b.iter(|| {
        test_fn(TinyText::try_from(value1).unwrap());
    });
}
