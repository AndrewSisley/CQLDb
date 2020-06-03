/*!
Testing helper library for cql_model storage types.
*/
use std::fmt::{ Debug };
use std::cmp::{ PartialEq };
use cql_model::{ CqlWritable, CqlReadable };

pub fn _1d_database_allows_for_single_point_read_writes<TStore: CqlWritable + CqlReadable>(db_location: &str, value: TStore::ValueType)
        where TStore::ValueType: Copy + Debug + PartialEq {
    let axis = [
        2,
    ];

    let point1 = [2];

    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &axis
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &point1,
        value
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<TStore>(
        db_location,
        &point1
    ).unwrap();

    assert_eq!(result1, value);
}

pub fn _4d_database_allows_for_single_point_read_writes<TStore: CqlWritable + CqlReadable>(db_location: &str, value: TStore::ValueType)
        where TStore::ValueType: Copy + Debug + PartialEq {
    let axis = [
        2,
        5,
        3,
        2,
    ];

    let point1 = [2, 4, 3, 1];

    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &axis
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TStore>(
        db_location,
        &point1[0..3],
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &point1,
        value
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<TStore>(
        db_location,
        &point1
    ).unwrap();

    assert_eq!(result1, value);
}
