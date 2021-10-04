mod constants;

use crate::constants::DATABASE_LOCATION;
use cql_model::{ CqlWritable, CqlReadable };
use cql_u64::{ U64, unpack_stream };
use serial_test::serial;
use futures::executor::{block_on, };
use futures;
use std::sync::{Arc, RwLock, LockResult, /*RwLockReadGuard,*/ RwLockWriteGuard};

const AXIS: [u64; 1] = [
    3,
];

#[test]
#[serial]
fn unchecked() {
    let db = lib::Database {
        resolution: u16::MAX as usize,
        thread_bins: vec![Arc::new(RwLock::new(()))]
    };

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &AXIS
    ).unwrap();

    block_on(lib::write_value_unchecked::<lib::Database, U64>(
        DATABASE_LOCATION,
        &db,
        &[5],
        1
    ));

    let result1 = cql_db::read_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &[5]
    ).unwrap();

    assert_eq!(result1, 1);
}

#[test]
#[serial]
fn unchecked2() {
    let db = lib::Database {
        resolution: u16::MAX as usize,
        thread_bins: vec![Arc::new(RwLock::new(())), Arc::new(RwLock::new(()))]
    };

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &AXIS
    ).unwrap();

    let f1 = lib::write_value_unchecked::<lib::Database, U64>(// fire these off in their own threads... test is just running sync atm
        DATABASE_LOCATION,
        &db,
        &[5],
        1
    );
    
    let f2 = lib::write_value_unchecked::<lib::Database, U64>(
        DATABASE_LOCATION,
        &db,
        &[5],
        2
    );
    
    let f3 = lib::write_value_unchecked::<lib::Database, U64>(
        DATABASE_LOCATION,
        &db,
        &[5],
        3
    );

    block_on(async {// todo...
        futures::join!(f1, f2, f3);
    });

    let result1 = cql_db::read_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &[5]
    ).unwrap();

    assert_eq!(result1, 3);
}