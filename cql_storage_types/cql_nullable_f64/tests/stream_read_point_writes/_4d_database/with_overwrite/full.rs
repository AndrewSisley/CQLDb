use serial_test::serial;
use crate::constants::DATABASE_LOCATION;
use cql_nullable_f64::{ NullableF64 };
use cql_storage_type_testing_lib::tests::stream_read_point_writes::_4d_database::with_overwrite::full;
use crate::unpack_nullable_f64_stream;

#[test]
#[serial]
fn unchecked() {
    full::unchecked::<NullableF64>(
        DATABASE_LOCATION,
        Some(11.1),
        Some(22.22),
        Some(33.333),
        Some(44.4444),
        Some(55.55555),
        Some(66.666666),
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
        Some(44.4444),
        Some(55.55555),
        Some(66.666666),
        &unpack_nullable_f64_stream
    );
}

#[test]
#[serial]
fn unchecked_write_unchecked_overwrite_checked_read() {
    full::unchecked_write_unchecked_overwrite_checked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(11.1),
        Some(22.22),
        Some(33.333),
        Some(44.4444),
        Some(55.55555),
        Some(66.666666),
        &unpack_nullable_f64_stream
    );
}

#[test]
#[serial]
fn unchecked_write_checked_overwrite_checked_read() {
    full::unchecked_write_checked_overwrite_checked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(11.1),
        Some(22.22),
        Some(33.333),
        Some(44.4444),
        Some(55.55555),
        Some(66.666666),
        &unpack_nullable_f64_stream
    );
}

#[test]
#[serial]
fn checked_write_unchecked_overwrite_checked_read() {
    full::checked_write_unchecked_overwrite_checked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(11.1),
        Some(22.22),
        Some(33.333),
        Some(44.4444),
        Some(55.55555),
        Some(66.666666),
        &unpack_nullable_f64_stream
    );
}

#[test]
#[serial]
fn checked_write_unchecked_overwrite_unchecked_read() {
    full::checked_write_unchecked_overwrite_unchecked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(11.1),
        Some(22.22),
        Some(33.333),
        Some(44.4444),
        Some(55.55555),
        Some(66.666666),
        &unpack_nullable_f64_stream
    );
}

#[test]
#[serial]
fn unchecked_write_checked_overwrite_unchecked_read() {
    full::unchecked_write_checked_overwrite_unchecked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(11.1),
        Some(22.22),
        Some(33.333),
        Some(44.4444),
        Some(55.55555),
        Some(66.666666),
        &unpack_nullable_f64_stream
    );
}

#[test]
#[serial]
fn checked_write_checked_overwrite_unchecked_read() {
    full::checked_write_checked_overwrite_unchecked_read::<NullableF64>(
        DATABASE_LOCATION,
        Some(11.1),
        Some(22.22),
        Some(33.333),
        Some(44.4444),
        Some(55.55555),
        Some(66.666666),
        &unpack_nullable_f64_stream
    );
}
