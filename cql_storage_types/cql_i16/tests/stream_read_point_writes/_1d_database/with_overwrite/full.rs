use serial_test::serial;
use crate::constants::DATABASE_LOCATION;
use cql_i16::{ I16 };
use cql_storage_type_testing_lib::tests::stream_read_point_writes::_1d_database::with_overwrite::full;
use crate::unpack_i16_stream;

#[test]
#[serial]
fn unchecked() {
    full::unchecked::<I16>(
        DATABASE_LOCATION,
        11,
        22,
        33,
        44,
        55,
        66,
        &unpack_i16_stream
    );
}

#[test]
#[serial]
fn checked() {
    full::checked::<I16>(
        DATABASE_LOCATION,
        11,
        22,
        33,
        44,
        55,
        66,
        &unpack_i16_stream
    );
}

#[test]
#[serial]
fn unchecked_write_unchecked_overwrite_checked_read() {
    full::unchecked_write_unchecked_overwrite_checked_read::<I16>(
        DATABASE_LOCATION,
        11,
        22,
        33,
        44,
        55,
        66,
        &unpack_i16_stream
    );
}

#[test]
#[serial]
fn unchecked_write_checked_overwrite_checked_read() {
    full::unchecked_write_checked_overwrite_checked_read::<I16>(
        DATABASE_LOCATION,
        11,
        22,
        33,
        44,
        55,
        66,
        &unpack_i16_stream
    );
}

#[test]
#[serial]
fn checked_write_unchecked_overwrite_checked_read() {
    full::checked_write_unchecked_overwrite_checked_read::<I16>(
        DATABASE_LOCATION,
        11,
        22,
        33,
        44,
        55,
        66,
        &unpack_i16_stream
    );
}

#[test]
#[serial]
fn checked_write_unchecked_overwrite_unchecked_read() {
    full::checked_write_unchecked_overwrite_unchecked_read::<I16>(
        DATABASE_LOCATION,
        11,
        22,
        33,
        44,
        55,
        66,
        &unpack_i16_stream
    );
}

#[test]
#[serial]
fn unchecked_write_checked_overwrite_unchecked_read() {
    full::unchecked_write_checked_overwrite_unchecked_read::<I16>(
        DATABASE_LOCATION,
        11,
        22,
        33,
        44,
        55,
        66,
        &unpack_i16_stream
    );
}

#[test]
#[serial]
fn checked_write_checked_overwrite_unchecked_read() {
    full::checked_write_checked_overwrite_unchecked_read::<I16>(
        DATABASE_LOCATION,
        11,
        22,
        33,
        44,
        55,
        66,
        &unpack_i16_stream
    );
}
