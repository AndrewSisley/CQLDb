use serial_test::serial;
use crate::constants::DATABASE_LOCATION;
use cql_tiny_text::{ TinyText };
use std::convert::TryFrom;
use cql_storage_type_testing_lib::tests::single_point_read_writes::_4d_database::with_overwrite::first_index;

#[test]
#[serial]
fn unchecked() {
    first_index::unchecked::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        TinyText::try_from("testValue2").unwrap()
    );
}

#[test]
#[serial]
fn checked() {
    first_index::checked::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        TinyText::try_from("testValue2").unwrap()
    );
}

#[test]
#[serial]
fn unchecked_write_unchecked_overwrite_checked_read() {
    first_index::unchecked_write_unchecked_overwrite_checked_read::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        TinyText::try_from("testValue2").unwrap()
    );
}

#[test]
#[serial]
fn unchecked_write_checked_overwrite_checked_read() {
    first_index::unchecked_write_checked_overwrite_checked_read::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        TinyText::try_from("testValue2").unwrap()
    );
}

#[test]
#[serial]
fn checked_write_unchecked_overwrite_checked_read() {
    first_index::checked_write_unchecked_overwrite_checked_read::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        TinyText::try_from("testValue2").unwrap()
    );
}

#[test]
#[serial]
fn checked_write_unchecked_overwrite_unchecked_read() {
    first_index::checked_write_unchecked_overwrite_unchecked_read::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        TinyText::try_from("testValue2").unwrap()
    );
}

#[test]
#[serial]
fn unchecked_write_checked_overwrite_unchecked_read() {
    first_index::unchecked_write_checked_overwrite_unchecked_read::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        TinyText::try_from("testValue2").unwrap()
    );
}

#[test]
#[serial]
fn checked_write_checked_overwrite_unchecked_read() {
    first_index::checked_write_checked_overwrite_unchecked_read::<TinyText>(
        DATABASE_LOCATION,
        TinyText::try_from("testValue1").unwrap(),
        TinyText::try_from("testValue2").unwrap()
    );
}
