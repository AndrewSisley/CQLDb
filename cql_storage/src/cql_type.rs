use std::fs::{File};

pub trait CqlType {
    type ValueType;

    fn create_db(db_location: &str) {
        let file = File::create(db_location).unwrap();
        file.set_len(0).unwrap();
    }

    fn grow_database(db_location: &str, size_to_grow: u64);
}

pub trait CqlWritable: CqlType {
    fn write_to_db(db_location: &str, value_location: u64, input_value: Self::ValueType);
}
