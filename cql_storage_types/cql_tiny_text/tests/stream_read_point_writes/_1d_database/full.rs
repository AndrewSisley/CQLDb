use serial_test::serial;
use crate::constants::DATABASE_LOCATION;
use cql_tiny_text::{ TinyText };
use std::convert::TryFrom;
use cql_storage_type_testing_lib::tests::stream_read_point_writes::_1d_database::full;
use crate::unpack_tiny_text_stream;

#[test]
#[serial]
fn unchecked() {
    full::unchecked::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        TinyText::try_from("testValue22").unwrap(),
        TinyText::try_from("testValue333").unwrap(),
        &unpack_tiny_text_stream
    );
}

#[test]
#[serial]
fn checked() {
    full::checked::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        TinyText::try_from("testValue22").unwrap(),
        TinyText::try_from("testValue333").unwrap(),
        &unpack_tiny_text_stream
    );
}

#[test]
#[serial]
fn unchecked_write_checked_read() {
    full::unchecked_write_checked_read::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        TinyText::try_from("testValue22").unwrap(),
        TinyText::try_from("testValue333").unwrap(),
        &unpack_tiny_text_stream
    );
}

#[test]
#[serial]
fn checked_write_unchecked_read() {
    full::checked_write_unchecked_read::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        TinyText::try_from("testValue22").unwrap(),
        TinyText::try_from("testValue333").unwrap(),
        &unpack_tiny_text_stream
    );
}
