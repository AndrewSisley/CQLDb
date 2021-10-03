use serial_test::serial;
use crate::constants::DATABASE_LOCATION;
use cql_f64::{ F64 };
use cql_storage_type_testing_lib::tests::single_point_read_writes::_1d_database::midpoint;

#[test]
#[serial]
fn unchecked() {
    midpoint::unchecked::<F64>(
        DATABASE_LOCATION,
        42.5
    );
}

#[test]
#[serial]
fn checked() {
    midpoint::checked::<F64>(
        DATABASE_LOCATION,
        42.5
    );
}

#[test]
#[serial]
fn unchecked_write_checked_read() {
    midpoint::unchecked_write_checked_read::<F64>(
        DATABASE_LOCATION,
        42.5
    );
}

#[test]
#[serial]
fn checked_write_unchecked_read() {
    midpoint::checked_write_unchecked_read::<F64>(
        DATABASE_LOCATION,
        42.5
    );
}
