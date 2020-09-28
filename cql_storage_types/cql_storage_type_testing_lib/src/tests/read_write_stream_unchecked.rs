use std::fmt::{ Debug };
use std::cmp::{ PartialEq };
use std::default::{ Default };
use std::io::{ Cursor, SeekFrom, Seek };
use cql_model::{ CqlWritable, CqlStreamReadable };

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
