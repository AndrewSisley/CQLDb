
use std::io;
use std::sync::{Arc, RwLock, LockResult, /*RwLockReadGuard,*/ RwLockWriteGuard};

use cql_model::{
    //CqlType,
    CqlWritable,
    /*CqlReadable,
    CqlStreamReadable*/
};
use cql_db;
use cql_db::positional::{
    calculate_position,
    write_value_to_position
};


#[derive(Clone)]
pub struct Database {
    thread_bins: Arc::<Vec<RwLock::<()>>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            thread_bins: Arc::new(vec![RwLock::new(()), RwLock::new(())])// todo: do properly...
        }
    }
}

pub trait CqlAsyncAccessor {
    fn get_write_lock(& self, position: u64) -> LockResult<RwLockWriteGuard<'_, ()>>;
}


pub async fn write_value_unchecked<TAccessor: CqlAsyncAccessor, TStore: CqlWritable>(
            db_location: &str,
            accessor: & dyn CqlAsyncAccessor,
            location: &[u64],
            value: TStore::ValueType
        ) -> io::Result<()> {//todo error type etc
    let position = calculate_position(db_location, location)?;// - expose? feels a tad internal...

    {
        let _lock = accessor.get_write_lock(position).unwrap();// can still deadlock if multiple read locks in same context I think - see doc, lock this down
        //cql_db::write_value_unchecked::<TStore>(&db_location, location, value).unwrap();//temp
        write_value_to_position::<TStore>(db_location, position, value)?; // feed position
    }
    Ok::<(), io::Error>(())
}

impl CqlAsyncAccessor for Database {
    fn get_write_lock(& self, position: u64) -> LockResult<RwLockWriteGuard<'_, ()>> {
        let thread_discriminator = (position % self.thread_bins.len() as u64) as usize; // temp cast - be smarter when sorting out resolution, this currently will be terrible for stream reads
        self.thread_bins[thread_discriminator].write()
    }
}

