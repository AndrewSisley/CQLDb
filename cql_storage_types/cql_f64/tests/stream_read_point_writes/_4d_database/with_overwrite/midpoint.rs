use serial_test::serial;
use crate::constants::DATABASE_LOCATION;
use cql_f64::{ F64 };
use cql_storage_type_testing_lib::tests::stream_read_point_writes::_4d_database::with_overwrite::midpoint;
use crate::unpack_f64_stream;

#[test]
#[serial]
fn unchecked() {
    midpoint::unchecked::<F64>(
        DATABASE_LOCATION,
        42.5,
        15.1,
        &unpack_f64_stream
    );
}

#[test]
#[serial]
fn checked() {
    midpoint::checked::<F64>(
        DATABASE_LOCATION,
        42.5,
        15.1,
        &unpack_f64_stream
    );
}

#[test]
#[serial]
fn unchecked_write_unchecked_overwrite_checked_read() {
    midpoint::unchecked_write_unchecked_overwrite_checked_read::<F64>(
        DATABASE_LOCATION,
        42.5,
        15.1,
        &unpack_f64_stream
    );
}

#[test]
#[serial]
fn unchecked_write_checked_overwrite_checked_read() {
    midpoint::unchecked_write_checked_overwrite_checked_read::<F64>(
        DATABASE_LOCATION,
        42.5,
        15.1,
        &unpack_f64_stream
    );
}

#[test]
#[serial]
fn checked_write_unchecked_overwrite_checked_read() {
    midpoint::checked_write_unchecked_overwrite_checked_read::<F64>(
        DATABASE_LOCATION,
        42.5,
        15.1,
        &unpack_f64_stream
    );
}

#[test]
#[serial]
fn checked_write_unchecked_overwrite_unchecked_read() {
    midpoint::checked_write_unchecked_overwrite_unchecked_read::<F64>(
        DATABASE_LOCATION,
        42.5,
        15.1,
        &unpack_f64_stream
    );
}

#[test]
#[serial]
fn unchecked_write_checked_overwrite_unchecked_read() {
    midpoint::unchecked_write_checked_overwrite_unchecked_read::<F64>(
        DATABASE_LOCATION,
        42.5,
        15.1,
        &unpack_f64_stream
    );
}

#[test]
#[serial]
fn checked_write_checked_overwrite_unchecked_read() {
    midpoint::checked_write_checked_overwrite_unchecked_read::<F64>(
        DATABASE_LOCATION,
        42.5,
        15.1,
        &unpack_f64_stream
    );
}
