use serial_test::serial;
use crate::constants::DATABASE_LOCATION;
use cql_tiny_text::{ TinyText };
use std::convert::TryFrom;
use cql_storage_type_testing_lib::tests::single_point_read_writes::_4d_database::midpoint;

#[test]
#[serial]
fn unchecked() {
    midpoint::unchecked::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap()
    );
}

#[test]
#[serial]
fn checked() {
    midpoint::checked::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap()
    );
}

#[test]
#[serial]
fn unchecked_write_checked_read() {
    midpoint::unchecked_write_checked_read::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap()
    );
}

#[test]
#[serial]
fn checked_write_unchecked_read() {
    midpoint::checked_write_unchecked_read::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap()
    );
}
