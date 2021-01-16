use std::fmt::{ Debug };
use std::cmp::{ PartialEq };
use cql_model::{ CqlWritable, CqlReadable };

const AXIS: [u64; 1] = [
    3,
];

pub fn unchecked<TStore: CqlWritable + CqlReadable>(db_location: &str, value1: TStore::ValueType, value2: TStore::ValueType, point: [u64; 1])
        where TStore::ValueType: Clone + Debug + PartialEq {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &point,
        value1.clone()
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &point,
        value2.clone()
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<TStore>(
        db_location,
        &point
    ).unwrap();

    assert_eq!(result1, value2);
}

pub fn checked<TStore: CqlWritable + CqlReadable>(db_location: &str, value1: TStore::ValueType, value2: TStore::ValueType, point: [u64; 1])
        where TStore::ValueType: Clone + Debug + PartialEq {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &point,
        value1.clone()
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &point,
        value2.clone()
    ).unwrap();

    let result1 = cql_db::read_value::<TStore>(
        db_location,
        &point
    ).unwrap();

    assert_eq!(result1, value2);
}

pub fn unchecked_write_unchecked_overwrite_checked_read<TStore: CqlWritable + CqlReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            point: [u64; 1]
        ) where TStore::ValueType: Clone + Debug + PartialEq {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &point,
        value1.clone()
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &point,
        value2.clone()
    ).unwrap();

    let result1 = cql_db::read_value::<TStore>(
        db_location,
        &point
    ).unwrap();

    assert_eq!(result1, value2);
}

pub fn unchecked_write_checked_overwrite_checked_read<TStore: CqlWritable + CqlReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            point: [u64; 1]
        ) where TStore::ValueType: Clone + Debug + PartialEq {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &point,
        value1.clone()
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &point,
        value2.clone()
    ).unwrap();

    let result1 = cql_db::read_value::<TStore>(
        db_location,
        &point
    ).unwrap();

    assert_eq!(result1, value2);
}

pub fn checked_write_unchecked_overwrite_checked_read<TStore: CqlWritable + CqlReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            point: [u64; 1]
        ) where TStore::ValueType: Clone + Debug + PartialEq {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &point,
        value1.clone()
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &point,
        value2.clone()
    ).unwrap();

    let result1 = cql_db::read_value::<TStore>(
        db_location,
        &point
    ).unwrap();

    assert_eq!(result1, value2);
}

pub fn checked_write_unchecked_overwrite_unchecked_read<TStore: CqlWritable + CqlReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            point: [u64; 1]
        ) where TStore::ValueType: Clone + Debug + PartialEq {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &point,
        value1.clone()
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &point,
        value2.clone()
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<TStore>(
        db_location,
        &point
    ).unwrap();

    assert_eq!(result1, value2);
}

pub fn unchecked_write_checked_overwrite_unchecked_read<TStore: CqlWritable + CqlReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            point: [u64; 1]
        ) where TStore::ValueType: Clone + Debug + PartialEq {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &point,
        value1.clone()
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &point,
        value2.clone()
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<TStore>(
        db_location,
        &point
    ).unwrap();

    assert_eq!(result1, value2);
}

pub fn checked_write_checked_overwrite_unchecked_read<TStore: CqlWritable + CqlReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            point: [u64; 1]
        ) where TStore::ValueType: Clone + Debug + PartialEq {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &point,
        value1.clone()
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &point,
        value2.clone()
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<TStore>(
        db_location,
        &point
    ).unwrap();

    assert_eq!(result1, value2);
}
