use serial_test::serial;
use crate::constants::DATABASE_LOCATION;
use cql_f64::{ F64 };
use cql_storage_type_testing_lib::tests::stream_read_point_writes::_1d_database::last_index;
use crate::unpack_f64_stream;

#[test]
#[serial]
fn unchecked() {
    last_index::unchecked::<F64>(
        DATABASE_LOCATION,
        42.5,
        &unpack_f64_stream
    );
}

#[test]
#[serial]
fn checked() {
    last_index::checked::<F64>(
        DATABASE_LOCATION,
        42.5,
        &unpack_f64_stream
    );
}

#[test]
#[serial]
fn unchecked_write_checked_read() {
    last_index::unchecked_write_checked_read::<F64>(
        DATABASE_LOCATION,
        42.5,
        &unpack_f64_stream
    );
}

#[test]
#[serial]
fn checked_write_unchecked_read() {
    last_index::checked_write_unchecked_read::<F64>(
        DATABASE_LOCATION,
        42.5,
        &unpack_f64_stream
    );
}
