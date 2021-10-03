use serial_test::serial;
use crate::constants::DATABASE_LOCATION;
use cql_f64::{ F64 };
use cql_storage_type_testing_lib::tests::stream_read_point_writes::_1d_database::full;
use crate::unpack_f64_stream;

#[test]
#[serial]
fn unchecked() {
    full::unchecked::<F64>(
        DATABASE_LOCATION,
        11.1,
        22.2,
        33.3,
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
        &unpack_f64_stream
    );
}

#[test]
#[serial]
fn unchecked_write_checked_read() {
    full::unchecked_write_checked_read::<F64>(
        DATABASE_LOCATION,
        11.1,
        22.2,
        33.3,
        &unpack_f64_stream
    );
}

#[test]
#[serial]
fn checked_write_unchecked_read() {
    full::checked_write_unchecked_read::<F64>(
        DATABASE_LOCATION,
        11.1,
        22.2,
        33.3,
        &unpack_f64_stream
    );
}
