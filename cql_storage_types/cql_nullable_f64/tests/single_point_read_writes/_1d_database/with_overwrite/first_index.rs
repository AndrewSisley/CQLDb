use serial_test::serial;
use crate::constants::DATABASE_LOCATION;
use cql_nullable_f64::{ NullableF64 };
use cql_storage_type_testing_lib::tests::single_point_read_writes::_1d_database::with_overwrite::first_index;

#[test]
#[serial]
fn unchecked() {
    first_index::unchecked::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.5),
        Some(15.001)
    );
}

#[test]
#[serial]
fn checked() {
    first_index::checked::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.5),
        Some(15.001)
    );
}

#[test]
#[serial]
fn unchecked_write_unchecked_overwrite_checked_read() {
    first_index::unchecked_write_unchecked_overwrite_checked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.5),
        Some(15.001)
    );
}

#[test]
#[serial]
fn unchecked_write_checked_overwrite_checked_read() {
    first_index::unchecked_write_checked_overwrite_checked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.5),
        Some(15.001)
    );
}

#[test]
#[serial]
fn checked_write_unchecked_overwrite_checked_read() {
    first_index::checked_write_unchecked_overwrite_checked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.5),
        Some(15.001)
    );
}

#[test]
#[serial]
fn checked_write_unchecked_overwrite_unchecked_read() {
    first_index::checked_write_unchecked_overwrite_unchecked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.5),
        Some(15.001)
    );
}

#[test]
#[serial]
fn unchecked_write_checked_overwrite_unchecked_read() {
    first_index::unchecked_write_checked_overwrite_unchecked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.5),
        Some(15.001)
    );
}

#[test]
#[serial]
fn checked_write_checked_overwrite_unchecked_read() {
    first_index::checked_write_checked_overwrite_unchecked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.5),
        Some(15.001)
    );
}
