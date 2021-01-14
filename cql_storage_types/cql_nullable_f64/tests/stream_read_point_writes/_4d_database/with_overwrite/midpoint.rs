use serial_test::serial;
use crate::constants::DATABASE_LOCATION;
use cql_nullable_f64::{ NullableF64 };
use cql_storage_type_testing_lib::tests::stream_read_point_writes::_4d_database::with_overwrite::midpoint;
use crate::unpack_nullable_f64_stream;

#[test]
#[serial]
fn unchecked() {
    midpoint::unchecked::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.5),
        Some(15.0001),
        &unpack_nullable_f64_stream
    );
}

#[test]
#[serial]
fn checked() {
    midpoint::checked::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.5),
        Some(15.0001),
        &unpack_nullable_f64_stream
    );
}

#[test]
#[serial]
fn unchecked_write_unchecked_overwrite_checked_read() {
    midpoint::unchecked_write_unchecked_overwrite_checked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.5),
        Some(15.0001),
        &unpack_nullable_f64_stream
    );
}

#[test]
#[serial]
fn unchecked_write_checked_overwrite_checked_read() {
    midpoint::unchecked_write_checked_overwrite_checked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.5),
        Some(15.0001),
        &unpack_nullable_f64_stream
    );
}

#[test]
#[serial]
fn checked_write_unchecked_overwrite_checked_read() {
    midpoint::checked_write_unchecked_overwrite_checked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.5),
        Some(15.0001),
        &unpack_nullable_f64_stream
    );
}

#[test]
#[serial]
fn checked_write_unchecked_overwrite_unchecked_read() {
    midpoint::checked_write_unchecked_overwrite_unchecked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.5),
        Some(15.0001),
        &unpack_nullable_f64_stream
    );
}

#[test]
#[serial]
fn unchecked_write_checked_overwrite_unchecked_read() {
    midpoint::unchecked_write_checked_overwrite_unchecked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.5),
        Some(15.0001),
        &unpack_nullable_f64_stream
    );
}

#[test]
#[serial]
fn checked_write_checked_overwrite_unchecked_read() {
    midpoint::checked_write_checked_overwrite_unchecked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(42.5),
        Some(15.0001),
        &unpack_nullable_f64_stream
    );
}
