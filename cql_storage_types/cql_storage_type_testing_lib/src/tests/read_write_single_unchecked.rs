use std::fmt::{ Debug };
use std::cmp::{ PartialEq };
use std::default::{ Default };
use cql_model::{ CqlWritable, CqlReadable };

pub fn _4d_database_allows_for_single_point_read_writes_given_multiple_values_and_overwrites<TStore: CqlWritable + CqlReadable>
        (db_location: &str, value1: TStore::ValueType, value2: TStore::ValueType, value3: TStore::ValueType, value4: TStore::ValueType)
        where TStore::ValueType: Copy + Debug + PartialEq + Default {
    let axis = [
        2,
        5,
        3,
        4,
    ];

    let point1 = [2, 4, 3, 1];
    let point2 = [1, 4, 3, 1];
    let point3 = [2, 1, 3, 1];
    let point4 = [2, 4, 3, 2];

    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &axis
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TStore>(
        db_location,
        &point1[0..3]
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TStore>(
        db_location,
        &point2[0..3]
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TStore>(
        db_location,
        &point3[0..3]
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &point1,
        value1
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &point2,
        value2
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &point3,
        value3
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<TStore>(
        db_location,
        &point1
    ).unwrap();

    let result2 = cql_db::read_value_unchecked::<TStore>(
        db_location,
        &point2
    ).unwrap();

    let result3 = cql_db::read_value_unchecked::<TStore>(
        db_location,
        &point3
    ).unwrap();

    let result4 = cql_db::read_value_unchecked::<TStore>(
        db_location,
        &point4
    ).unwrap();

    assert_eq!(result1, value1);
    assert_eq!(result2, value2);
    assert_eq!(result3, value3);
    assert_eq!(result4, Default::default());

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &point2,
        value4
    ).unwrap();

    let result5 = cql_db::read_value_unchecked::<TStore>(
        db_location,
        &point2
    ).unwrap();

    assert_eq!(result5, value4);
}
