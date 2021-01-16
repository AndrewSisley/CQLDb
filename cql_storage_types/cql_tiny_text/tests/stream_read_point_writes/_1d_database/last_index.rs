use serial_test::serial;
use crate::constants::DATABASE_LOCATION;
use cql_tiny_text::{ TinyText };
use std::convert::TryFrom;
use cql_storage_type_testing_lib::tests::stream_read_point_writes::_1d_database::last_index;
use crate::unpack_tiny_text_stream;

#[test]
#[serial]
fn unchecked() {
    last_index::unchecked::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        &unpack_tiny_text_stream
    );
}

#[test]
#[serial]
fn checked() {
    last_index::checked::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        &unpack_tiny_text_stream
    );
}

#[test]
#[serial]
fn unchecked_write_checked_read() {
    last_index::unchecked_write_checked_read::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        &unpack_tiny_text_stream
    );
}

#[test]
#[serial]
fn checked_write_unchecked_read() {
    last_index::checked_write_unchecked_read::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        &unpack_tiny_text_stream
    );
}
