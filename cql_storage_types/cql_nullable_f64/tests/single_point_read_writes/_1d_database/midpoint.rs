use serial_test::serial;
use crate::constants::DATABASE_LOCATION;
use cql_nullable_f64::{ NullableF64 };
use cql_storage_type_testing_lib::tests::single_point_read_writes::_1d_database::midpoint;

#[test]
#[serial]
fn unchecked() {
    midpoint::unchecked::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.5)
    );
}

#[test]
#[serial]
fn checked() {
    midpoint::checked::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.5)
    );
}

#[test]
#[serial]
fn unchecked_write_checked_read() {
    midpoint::unchecked_write_checked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.5)
    );
}

#[test]
#[serial]
fn checked_write_unchecked_read() {
    midpoint::checked_write_unchecked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.5)
    );
}
