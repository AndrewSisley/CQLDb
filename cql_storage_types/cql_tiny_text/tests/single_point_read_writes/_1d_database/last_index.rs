use serial_test::serial;
use crate::constants::DATABASE_LOCATION;
use cql_tiny_text::{ TinyText };
use std::convert::TryFrom;
use cql_storage_type_testing_lib::tests::single_point_read_writes::_1d_database::last_index;

#[test]
#[serial]
fn unchecked() {
    last_index::unchecked::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap()
    );
}

#[test]
#[serial]
fn checked() {
    last_index::checked::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap()
    );
}

#[test]
#[serial]
fn unchecked_write_checked_read() {
    last_index::unchecked_write_checked_read::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap()
    );
}

#[test]
#[serial]
fn checked_write_unchecked_read() {
    last_index::checked_write_unchecked_read::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap()
    );
}
