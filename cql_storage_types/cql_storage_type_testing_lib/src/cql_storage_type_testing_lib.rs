/*!
Testing helper library for cql_model storage types.
*/
use std::fmt::{ Debug };
use std::cmp::{ PartialEq };
use std::default::{ Default };
use std::io::{ Cursor, SeekFrom, Seek };
use cql_model::{ CqlWritable, CqlReadable, CqlStreamReadable };

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

pub fn _1d_database_allows_for_stream_reads<TStore: CqlWritable + CqlStreamReadable, TUnpackStream>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            value3: TStore::ValueType,
            unpack_stream: &dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Copy + Debug + PartialEq + Default {
    let base_point = [2];
    const N_VALUES_TO_READ: usize = 3;

    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &[10]
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &base_point,
        value1
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &[base_point[0] + 1],
        value2
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &[base_point[0] + 2],
        value3
    ).unwrap();

    let mut result = [Default::default(); N_VALUES_TO_READ];
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream_unchecked::<TStore>(
        db_location,
        &mut stream,
        &base_point,
        N_VALUES_TO_READ as u64
    ).unwrap();

    stream.seek(SeekFrom::Start(0)).unwrap();

    unpack_stream(&mut stream, N_VALUES_TO_READ, &mut result);

    assert_eq!(result[0], value1);
    assert_eq!(result[1], value2);
    assert_eq!(result[2], value3);
}

pub fn _4d_database_allows_for_stream_reads<TStore: CqlWritable + CqlStreamReadable, TUnpackStream>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            value3: TStore::ValueType,
            unpack_stream: &dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Copy + Debug + PartialEq + Default {
    let base_point = [1, 1, 1, 2];
    const N_VALUES_TO_READ: usize = 4;

    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &[1, 1, 1, 10]
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TStore>(
        db_location,
        &base_point[0..3]
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &base_point,
        value1
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &[1, 1, 1, base_point[3] + 1],
        value2
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &[1, 1, 1, base_point[3] + 3],
        value3
    ).unwrap();

    let mut result = [Default::default(); N_VALUES_TO_READ];
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream_unchecked::<TStore>(
        db_location,
        &mut stream,
        &base_point,
        N_VALUES_TO_READ as u64
    ).unwrap();

    stream.seek(SeekFrom::Start(0)).unwrap();

    unpack_stream(&mut stream, N_VALUES_TO_READ, &mut result);

    assert_eq!(result[0], value1);
    assert_eq!(result[1], value2);
    assert_eq!(result[2], Default::default());
    assert_eq!(result[3], value3);
}
