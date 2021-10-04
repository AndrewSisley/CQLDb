
use std::io;
use std::sync::{Arc, RwLock, LockResult, /*RwLockReadGuard,*/ RwLockWriteGuard};

use cql_model::{
    //CqlType,
    CqlWritable,
    /*CqlReadable,
    CqlStreamReadable*/
};
use cql_db;


pub struct Database {
    pub resolution: usize,
    pub thread_bins: Vec<Arc::<RwLock::<()>>>,
}

pub trait CqlAsyncAccessor {
    //fn get_thread_discriminator(& self, position: u64) -> usize;// is a terrible interface

    fn get_write_lock(& self, position: u64) -> LockResult<RwLockWriteGuard<'_, ()>>;
}


pub async fn write_value_unchecked<TAccessor: CqlAsyncAccessor, TStore: CqlWritable>(
            db_location: &str,
            accessor: & dyn CqlAsyncAccessor,
            location: &[u64],
            value: TStore::ValueType
        ) -> io::Result<()> {//todo error type etc
    let position: u64 = 1234; // calculate_position - expose? feels a tad internal...

    {
        let _lock = accessor.get_write_lock(position).unwrap();// can still deadlock if multiple read locks in same context I think - see doc, lock this down
        cql_db::write_value_unchecked::<TStore>(&db_location, location, value).unwrap();//temp
        //cql_db::write_value_unchecked::<TStore>(&db.location, position, value) // feed position
    }
    Ok::<(), io::Error>(())
}

impl CqlAsyncAccessor for Database {
    fn get_write_lock(& self, position: u64) -> LockResult<RwLockWriteGuard<'_, ()>> {
        let thread_discriminator = (position % self.thread_bins.len() as u64) as usize; // temp cast - be smarter when sorting out resolution
        self.thread_bins[thread_discriminator].write()
    }
}

