use std::fmt::{ Debug };
use std::cmp::{ PartialEq };
use std::io::{ Cursor, SeekFrom, Seek };
use cql_model::{ CqlWritable, CqlStreamReadable };

const AXIS: [u64; 1] = [
    3,
];

const N_VALUES_TO_READ: usize = 3;
const POINT1: [u64; 1] = [1];
const POINT2: [u64; 1] = [2];
const POINT3: [u64; 1] = [3];

pub fn unchecked<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            value3: TStore::ValueType,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Clone + Debug + PartialEq + Default {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &POINT1,
        value1.clone()
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &POINT2,
        value2.clone()
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &POINT3,
        value3.clone()
    ).unwrap();

    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    result.resize_with(N_VALUES_TO_READ, Default::default);
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream_unchecked::<TStore>(
        db_location,
        &mut stream,
        &POINT1,
        N_VALUES_TO_READ as u64
    ).unwrap();

    stream.seek(SeekFrom::Start(0)).unwrap();

    unpack_stream(&mut stream, N_VALUES_TO_READ, &mut result);

    assert_eq!(result[0], value1);
    assert_eq!(result[1], value2);
    assert_eq!(result[2], value3);
}

pub fn checked<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            value3: TStore::ValueType,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Clone + Debug + PartialEq + Default {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &POINT1,
        value1.clone()
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &POINT2,
        value2.clone()
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &POINT3,
        value3.clone()
    ).unwrap();

    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    result.resize_with(N_VALUES_TO_READ, Default::default);
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream::<TStore>(
        db_location,
        &mut stream,
        &POINT1,
        N_VALUES_TO_READ as u64
    ).unwrap();

    stream.seek(SeekFrom::Start(0)).unwrap();

    unpack_stream(&mut stream, N_VALUES_TO_READ, &mut result);

    assert_eq!(result[0], value1);
    assert_eq!(result[1], value2);
    assert_eq!(result[2], value3);
}

pub fn unchecked_write_checked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            value3: TStore::ValueType,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Clone + Debug + PartialEq + Default {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &POINT1,
        value1.clone()
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &POINT2,
        value2.clone()
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &POINT3,
        value3.clone()
    ).unwrap();

    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    result.resize_with(N_VALUES_TO_READ, Default::default);
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream::<TStore>(
        db_location,
        &mut stream,
        &POINT1,
        N_VALUES_TO_READ as u64
    ).unwrap();

    stream.seek(SeekFrom::Start(0)).unwrap();

    unpack_stream(&mut stream, N_VALUES_TO_READ, &mut result);

    assert_eq!(result[0], value1);
    assert_eq!(result[1], value2);
    assert_eq!(result[2], value3);
}

pub fn checked_write_unchecked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            value3: TStore::ValueType,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Clone + Debug + PartialEq + Default {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &POINT1,
        value1.clone()
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &POINT2,
        value2.clone()
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &POINT3,
        value3.clone()
    ).unwrap();

    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    result.resize_with(N_VALUES_TO_READ, Default::default);
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream_unchecked::<TStore>(
        db_location,
        &mut stream,
        &POINT1,
        N_VALUES_TO_READ as u64
    ).unwrap();

    stream.seek(SeekFrom::Start(0)).unwrap();

    unpack_stream(&mut stream, N_VALUES_TO_READ, &mut result);

    assert_eq!(result[0], value1);
    assert_eq!(result[1], value2);
    assert_eq!(result[2], value3);
}
