use serial_test::serial;
use crate::constants::DATABASE_LOCATION;
use cql_tiny_text::{ TinyText };
use std::convert::TryFrom;
use cql_storage_type_testing_lib::tests::stream_read_point_writes::_4d_database::with_overwrite::midpoint;
use crate::unpack_tiny_text_stream;

#[test]
#[serial]
fn unchecked() {
    midpoint::unchecked::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        TinyText::try_from("testValue2").unwrap(),
        &unpack_tiny_text_stream
    );
}

#[test]
#[serial]
fn checked() {
    midpoint::checked::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        TinyText::try_from("testValue2").unwrap(),
        &unpack_tiny_text_stream
    );
}

#[test]
#[serial]
fn unchecked_write_unchecked_overwrite_checked_read() {
    midpoint::unchecked_write_unchecked_overwrite_checked_read::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        TinyText::try_from("testValue2").unwrap(),
        &unpack_tiny_text_stream
    );
}

#[test]
#[serial]
fn unchecked_write_checked_overwrite_checked_read() {
    midpoint::unchecked_write_checked_overwrite_checked_read::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        TinyText::try_from("testValue2").unwrap(),
        &unpack_tiny_text_stream
    );
}

#[test]
#[serial]
fn checked_write_unchecked_overwrite_checked_read() {
    midpoint::checked_write_unchecked_overwrite_checked_read::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        TinyText::try_from("testValue2").unwrap(),
        &unpack_tiny_text_stream
    );
}

#[test]
#[serial]
fn checked_write_unchecked_overwrite_unchecked_read() {
    midpoint::checked_write_unchecked_overwrite_unchecked_read::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        TinyText::try_from("testValue2").unwrap(),
        &unpack_tiny_text_stream
    );
}

#[test]
#[serial]
fn unchecked_write_checked_overwrite_unchecked_read() {
    midpoint::unchecked_write_checked_overwrite_unchecked_read::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        TinyText::try_from("testValue2").unwrap(),
        &unpack_tiny_text_stream
    );
}

#[test]
#[serial]
fn checked_write_checked_overwrite_unchecked_read() {
    midpoint::checked_write_checked_overwrite_unchecked_read::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        TinyText::try_from("testValue2").unwrap(),
        &unpack_tiny_text_stream
    );
}
