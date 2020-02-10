use std::io::{ Write };

pub trait CqlType {
    type ValueType;
    const VALUE_SIZE: usize;
}

pub trait CqlWritable: CqlType {
    fn write_to_db(db_location: &str, value_location: u64, input_value: Self::ValueType);
}

pub trait CqlReadable: CqlType {
    fn read_from_db(db_location: &str, value_location: u64) -> Self::ValueType;
}

pub trait CqlStreamReadable: CqlType {
    fn read_to_stream(db_location: &str, stream: &mut dyn Write, value_location: u64, n_values: u64);
}
