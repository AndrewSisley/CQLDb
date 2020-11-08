use serial_test::serial;
use crate::constants::DATABASE_LOCATION;
use cql_i16::{ I16 };
use cql_storage_type_testing_lib::tests::single_point_read_writes::_4d_database::with_overwrite::last_index;

#[test]
#[serial]
fn unchecked() {
    last_index::unchecked::<I16>(
        DATABASE_LOCATION,
        42,
        15
    );
}

#[test]
#[serial]
fn checked() {
    last_index::checked::<I16>(
        DATABASE_LOCATION,
        42,
        15
    );
}

#[test]
#[serial]
fn unchecked_write_unchecked_overwrite_checked_read() {
    last_index::unchecked_write_unchecked_overwrite_checked_read::<I16>(
        DATABASE_LOCATION,
        42,
        15
    );
}

#[test]
#[serial]
fn unchecked_write_checked_overwrite_checked_read() {
    last_index::unchecked_write_checked_overwrite_checked_read::<I16>(
        DATABASE_LOCATION,
        42,
        15
    );
}

#[test]
#[serial]
fn checked_write_unchecked_overwrite_checked_read() {
    last_index::checked_write_unchecked_overwrite_checked_read::<I16>(
        DATABASE_LOCATION,
        42,
        15
    );
}

#[test]
#[serial]
fn checked_write_unchecked_overwrite_unchecked_read() {
    last_index::checked_write_unchecked_overwrite_unchecked_read::<I16>(
        DATABASE_LOCATION,
        42,
        15
    );
}

#[test]
#[serial]
fn unchecked_write_checked_overwrite_unchecked_read() {
    last_index::unchecked_write_checked_overwrite_unchecked_read::<I16>(
        DATABASE_LOCATION,
        42,
        15
    );
}

#[test]
#[serial]
fn checked_write_checked_overwrite_unchecked_read() {
    last_index::checked_write_checked_overwrite_unchecked_read::<I16>(
        DATABASE_LOCATION,
        42,
        15
    );
}
