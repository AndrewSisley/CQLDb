use std::fmt::{ Debug };
use std::cmp::{ PartialEq };
use std::io::{ Cursor, SeekFrom, Seek };
use cql_model::{ CqlWritable, CqlStreamReadable };

const AXIS: [u64; 4] = [
    3,
    5,
    7,
    9,
];

const N_VALUES_TO_READ: usize = 9;
const POINT1: [u64; 4] = [2, 3, 4, 1];
const POINT2: [u64; 4] = [2, 3, 4, 5];
const POINT3: [u64; 4] = [2, 3, 4, 9];

pub fn unchecked<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            value3: TStore::ValueType,
            value4: TStore::ValueType,
            value5: TStore::ValueType,
            value6: TStore::ValueType,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Clone + Debug + PartialEq + Default {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TStore>(
        db_location,
        &POINT1[0..3],
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

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &POINT1,
        value4.clone()
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &POINT2,
        value5.clone()
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &POINT3,
        value6.clone()
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

    assert_eq!(result[0], value4);
    assert_eq!(result[4], value5);
    assert_eq!(result[8], value6);
}

pub fn checked<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            value3: TStore::ValueType,
            value4: TStore::ValueType,
            value5: TStore::ValueType,
            value6: TStore::ValueType,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Clone + Debug + PartialEq + Default {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TStore>(
        db_location,
        &POINT1[0..3],
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

    cql_db::write_value::<TStore>(
        db_location,
        &POINT1,
        value4.clone()
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &POINT2,
        value5.clone()
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &POINT3,
        value6.clone()
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

    assert_eq!(result[0], value4);
    assert_eq!(result[4], value5);
    assert_eq!(result[8], value6);
}

pub fn unchecked_write_unchecked_overwrite_checked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            value3: TStore::ValueType,
            value4: TStore::ValueType,
            value5: TStore::ValueType,
            value6: TStore::ValueType,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Clone + Debug + PartialEq + Default {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TStore>(
        db_location,
        &POINT1[0..3],
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

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &POINT1,
        value4.clone()
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &POINT2,
        value5.clone()
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &POINT3,
        value6.clone()
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

    assert_eq!(result[0], value4);
    assert_eq!(result[4], value5);
    assert_eq!(result[8], value6);
}

pub fn unchecked_write_checked_overwrite_checked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            value3: TStore::ValueType,
            value4: TStore::ValueType,
            value5: TStore::ValueType,
            value6: TStore::ValueType,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Clone + Debug + PartialEq + Default {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TStore>(
        db_location,
        &POINT1[0..3],
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

    cql_db::write_value::<TStore>(
        db_location,
        &POINT1,
        value4.clone()
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &POINT2,
        value5.clone()
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &POINT3,
        value6.clone()
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

    assert_eq!(result[0], value4);
    assert_eq!(result[4], value5);
    assert_eq!(result[8], value6);
}

pub fn checked_write_unchecked_overwrite_checked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            value3: TStore::ValueType,
            value4: TStore::ValueType,
            value5: TStore::ValueType,
            value6: TStore::ValueType,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Clone + Debug + PartialEq + Default {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TStore>(
        db_location,
        &POINT1[0..3],
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

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &POINT1,
        value4.clone()
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &POINT2,
        value5.clone()
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &POINT3,
        value6.clone()
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

    assert_eq!(result[0], value4);
    assert_eq!(result[4], value5);
    assert_eq!(result[8], value6);
}

pub fn checked_write_unchecked_overwrite_unchecked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            value3: TStore::ValueType,
            value4: TStore::ValueType,
            value5: TStore::ValueType,
            value6: TStore::ValueType,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Clone + Debug + PartialEq + Default {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TStore>(
        db_location,
        &POINT1[0..3],
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

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &POINT1,
        value4.clone()
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &POINT2,
        value5.clone()
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &POINT3,
        value6.clone()
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

    assert_eq!(result[0], value4);
    assert_eq!(result[4], value5);
    assert_eq!(result[8], value6);
}

pub fn unchecked_write_checked_overwrite_unchecked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            value3: TStore::ValueType,
            value4: TStore::ValueType,
            value5: TStore::ValueType,
            value6: TStore::ValueType,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Clone + Debug + PartialEq + Default {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TStore>(
        db_location,
        &POINT1[0..3],
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

    cql_db::write_value::<TStore>(
        db_location,
        &POINT1,
        value4.clone()
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &POINT2,
        value5.clone()
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &POINT3,
        value6.clone()
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

    assert_eq!(result[0], value4);
    assert_eq!(result[4], value5);
    assert_eq!(result[8], value6);
}

pub fn checked_write_checked_overwrite_unchecked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            value3: TStore::ValueType,
            value4: TStore::ValueType,
            value5: TStore::ValueType,
            value6: TStore::ValueType,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Clone + Debug + PartialEq + Default {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TStore>(
        db_location,
        &POINT1[0..3],
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

    cql_db::write_value::<TStore>(
        db_location,
        &POINT1,
        value4.clone()
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &POINT2,
        value5.clone()
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &POINT3,
        value6.clone()
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

    assert_eq!(result[0], value4);
    assert_eq!(result[4], value5);
    assert_eq!(result[8], value6);
}
