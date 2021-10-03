use serial_test::serial;
use crate::constants::DATABASE_LOCATION;
use cql_f64::{ F64 };
use cql_storage_type_testing_lib::tests::stream_read_point_writes::_4d_database::with_overwrite::full;
use crate::unpack_f64_stream;

#[test]
#[serial]
fn unchecked() {
    full::unchecked::<F64>(
        DATABASE_LOCATION,
        11.1,
        22.2,
        33.3,
        44.4,
        55.5,
        66.6,
        &unpack_f64_stream
    );
}

#[test]
#[serial]
fn checked() {
    full::checked::<F64>(
        DATABASE_LOCATION,
        11.1,
        22.2,
        33.3,
        44.4,
        55.5,
        66.6,
        &unpack_f64_stream
    );
}

#[test]
#[serial]
fn unchecked_write_unchecked_overwrite_checked_read() {
    full::unchecked_write_unchecked_overwrite_checked_read::<F64>(
        DATABASE_LOCATION,
        11.1,
        22.2,
        33.3,
        44.4,
        55.5,
        66.6,
        &unpack_f64_stream
    );
}

#[test]
#[serial]
fn unchecked_write_checked_overwrite_checked_read() {
    full::unchecked_write_checked_overwrite_checked_read::<F64>(
        DATABASE_LOCATION,
        11.1,
        22.2,
        33.3,
        44.4,
        55.5,
        66.6,
        &unpack_f64_stream
    );
}

#[test]
#[serial]
fn checked_write_unchecked_overwrite_checked_read() {
    full::checked_write_unchecked_overwrite_checked_read::<F64>(
        DATABASE_LOCATION,
        11.1,
        22.2,
        33.3,
        44.4,
        55.5,
        66.6,
        &unpack_f64_stream
    );
}

#[test]
#[serial]
fn checked_write_unchecked_overwrite_unchecked_read() {
    full::checked_write_unchecked_overwrite_unchecked_read::<F64>(
        DATABASE_LOCATION,
        11.1,
        22.2,
        33.3,
        44.4,
        55.5,
        66.6,
        &unpack_f64_stream
    );
}

#[test]
#[serial]
fn unchecked_write_checked_overwrite_unchecked_read() {
    full::unchecked_write_checked_overwrite_unchecked_read::<F64>(
        DATABASE_LOCATION,
        11.1,
        22.2,
        33.3,
        44.4,
        55.5,
        66.6,
        &unpack_f64_stream
    );
}

#[test]
#[serial]
fn checked_write_checked_overwrite_unchecked_read() {
    full::checked_write_checked_overwrite_unchecked_read::<F64>(
        DATABASE_LOCATION,
        11.1,
        22.2,
        33.3,
        44.4,
        55.5,
        66.6,
        &unpack_f64_stream
    );
}
