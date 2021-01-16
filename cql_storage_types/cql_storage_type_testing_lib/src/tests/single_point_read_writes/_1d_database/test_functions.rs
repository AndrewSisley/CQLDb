use std::fmt::{ Debug };
use std::cmp::{ PartialEq };
use cql_model::{ CqlWritable, CqlReadable };

const AXIS: [u64; 1] = [
    3,
];

pub fn unchecked<TStore: CqlWritable + CqlReadable>(db_location: &str, value: TStore::ValueType, point: [u64; 1])
        where TStore::ValueType: Clone + Debug + PartialEq {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &point,
        value.clone()
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<TStore>(
        db_location,
        &point
    ).unwrap();

    assert_eq!(result1, value);
}

pub fn checked<TStore: CqlWritable + CqlReadable>(db_location: &str, value: TStore::ValueType, point: [u64; 1])
        where TStore::ValueType: Clone + Debug + PartialEq {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &point,
        value.clone()
    ).unwrap();

    let result1 = cql_db::read_value::<TStore>(
        db_location,
        &point
    ).unwrap();

    assert_eq!(result1, value);
}

pub fn unchecked_write_checked_read<TStore: CqlWritable + CqlReadable>(db_location: &str, value: TStore::ValueType, point: [u64; 1])
        where TStore::ValueType: Clone + Debug + PartialEq {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &point,
        value.clone()
    ).unwrap();

    let result1 = cql_db::read_value::<TStore>(
        db_location,
        &point
    ).unwrap();

    assert_eq!(result1, value);
}

pub fn checked_write_unchecked_read<TStore: CqlWritable + CqlReadable>(db_location: &str, value: TStore::ValueType, point: [u64; 1])
        where TStore::ValueType: Clone + Debug + PartialEq {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &point,
        value.clone()
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<TStore>(
        db_location,
        &point
    ).unwrap();

    assert_eq!(result1, value);
}
