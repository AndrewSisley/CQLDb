mod constants;

use crate::constants::DATABASE_LOCATION;
use cql_model::{ CqlWritable, CqlReadable };
use cql_u64::{ U64, unpack_stream };
use serial_test::serial;
use futures::executor::{block_on, };
use futures;
use std::thread;
use std::sync::{Arc, RwLock, LockResult, /*RwLockReadGuard,*/ RwLockWriteGuard};

const AXIS: [u64; 1] = [
    3,
];

#[test]
#[serial]
fn unchecked() {
    let db = lib::Database::new();

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
    /*let db = lib::Database {
        resolution: u16::MAX as usize,
        thread_bins: vec![Arc::new(RwLock::new(())), Arc::new(RwLock::new(()))]
    };*/
    let db = lib::Database::new();

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &AXIS
    ).unwrap();

    

    let db1 = db.clone();
    let t1 = thread::spawn(move || {
        let f1 = lib::write_value_unchecked::<lib::Database, U64>(
            DATABASE_LOCATION,
            &db1,
            &[5],
            1
        );
        
        let f2 = lib::write_value_unchecked::<lib::Database, U64>(
            DATABASE_LOCATION,
            &db1,
            &[5],
            2
        );
        
        let f3 = lib::write_value_unchecked::<lib::Database, U64>(
            DATABASE_LOCATION,
            &db1,
            &[5],
            3
        );

        block_on(async {// todo...
            futures::join!(f1, f2, f3);
        });
    });


    let db2 = db.clone();
    let t2 = thread::spawn(move || {
        let f1 = lib::write_value_unchecked::<lib::Database, U64>(
            DATABASE_LOCATION,
            &db2,
            &[5],
            1
        );
        
        let f2 = lib::write_value_unchecked::<lib::Database, U64>(
            DATABASE_LOCATION,
            &db2,
            &[5],
            2
        );
        
        let f3 = lib::write_value_unchecked::<lib::Database, U64>(
            DATABASE_LOCATION,
            &db2,
            &[5],
            3
        );

        block_on(async {// todo...
            futures::join!(f1, f2, f3);
        });
    });

    t1.join();
    t2.join();

    

    let result1 = cql_db::read_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &[5]
    ).unwrap();

    assert_eq!(result1, 3);
}