use serial_test::serial;
use crate::constants::DATABASE_LOCATION;
use cql_i16::{ I16 };
use cql_storage_type_testing_lib::tests::single_point_read_writes::_1d_database::with_overwrite::midpoint;

#[test]
#[serial]
fn unchecked() {
    midpoint::unchecked::<I16>(
        DATABASE_LOCATION,
        42,
        15
    );
}

#[test]
#[serial]
fn checked() {
    midpoint::checked::<I16>(
        DATABASE_LOCATION,
        42,
        15
    );
}

#[test]
#[serial]
fn unchecked_write_unchecked_overwrite_checked_read() {
    midpoint::unchecked_write_unchecked_overwrite_checked_read::<I16>(
        DATABASE_LOCATION,
        42,
        15
    );
}

#[test]
#[serial]
fn unchecked_write_checked_overwrite_checked_read() {
    midpoint::unchecked_write_checked_overwrite_checked_read::<I16>(
        DATABASE_LOCATION,
        42,
        15
    );
}

#[test]
#[serial]
fn checked_write_unchecked_overwrite_checked_read() {
    midpoint::checked_write_unchecked_overwrite_checked_read::<I16>(
        DATABASE_LOCATION,
        42,
        15
    );
}

#[test]
#[serial]
fn checked_write_unchecked_overwrite_unchecked_read() {
    midpoint::checked_write_unchecked_overwrite_unchecked_read::<I16>(
        DATABASE_LOCATION,
        42,
        15
    );
}

#[test]
#[serial]
fn unchecked_write_checked_overwrite_unchecked_read() {
    midpoint::unchecked_write_checked_overwrite_unchecked_read::<I16>(
        DATABASE_LOCATION,
        42,
        15
    );
}

#[test]
#[serial]
fn checked_write_checked_overwrite_unchecked_read() {
    midpoint::checked_write_checked_overwrite_unchecked_read::<I16>(
        DATABASE_LOCATION,
        42,
        15
    );
}
