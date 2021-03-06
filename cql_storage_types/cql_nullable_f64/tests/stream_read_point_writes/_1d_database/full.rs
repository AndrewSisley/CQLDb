use serial_test::serial;
use crate::constants::DATABASE_LOCATION;
use cql_nullable_f64::{ NullableF64 };
use cql_storage_type_testing_lib::tests::stream_read_point_writes::_1d_database::full;
use crate::unpack_nullable_f64_stream;

#[test]
#[serial]
fn unchecked() {
    full::unchecked::<NullableF64>(
        DATABASE_LOCATION,
        Some(11.1),
        Some(22.22),
        Some(33.333),
        &unpack_nullable_f64_stream
    );
}

#[test]
#[serial]
fn checked() {
    full::checked::<NullableF64>(
        DATABASE_LOCATION,
        Some(11.1),
        Some(22.22),
        Some(33.333),
        &unpack_nullable_f64_stream
    );
}

#[test]
#[serial]
fn unchecked_write_checked_read() {
    full::unchecked_write_checked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(11.1),
        Some(22.22),
        Some(33.333),
        &unpack_nullable_f64_stream
    );
}

#[test]
#[serial]
fn checked_write_unchecked_read() {
    full::checked_write_unchecked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(11.1),
        Some(22.22),
        Some(33.333),
        &unpack_nullable_f64_stream
    );
}
