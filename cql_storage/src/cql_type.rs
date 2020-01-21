use std::fs::{File};

pub trait CqlType {
    fn create_db(db_location: &str) {
        let file = File::create(db_location).unwrap();
        file.set_len(0).unwrap();
    }

    fn grow_database(db_location: &str, size_to_grow: u64);
}
