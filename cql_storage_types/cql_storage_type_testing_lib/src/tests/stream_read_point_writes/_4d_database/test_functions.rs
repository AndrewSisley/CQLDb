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

pub fn unchecked<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value: TStore::ValueType,
            point: [u64; 4],
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Copy + Debug + PartialEq + Default {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TStore>(
        db_location,
        &point[0..3],
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &point,
        value
    ).unwrap();

    const N_VALUES_TO_READ: usize = 1;
    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    result.resize_with(N_VALUES_TO_READ, Default::default);
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream_unchecked::<TStore>(
        db_location,
        &mut stream,
        &point,
        N_VALUES_TO_READ as u64
    ).unwrap();

    stream.seek(SeekFrom::Start(0)).unwrap();

    unpack_stream(&mut stream, N_VALUES_TO_READ, &mut result);

    assert_eq!(result[0], value);
}

pub fn checked<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value: TStore::ValueType,
            point: [u64; 4],
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Copy + Debug + PartialEq + Default {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TStore>(
        db_location,
        &point[0..3],
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &point,
        value
    ).unwrap();

    const N_VALUES_TO_READ: usize = 1;
    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    result.resize_with(N_VALUES_TO_READ, Default::default);
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream::<TStore>(
        db_location,
        &mut stream,
        &point,
        N_VALUES_TO_READ as u64
    ).unwrap();

    stream.seek(SeekFrom::Start(0)).unwrap();

    unpack_stream(&mut stream, N_VALUES_TO_READ, &mut result);

    assert_eq!(result[0], value);
}

pub fn unchecked_write_checked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value: TStore::ValueType,
            point: [u64; 4],
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Copy + Debug + PartialEq + Default {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TStore>(
        db_location,
        &point[0..3],
    ).unwrap();

    cql_db::write_value_unchecked::<TStore>(
        db_location,
        &point,
        value
    ).unwrap();

    const N_VALUES_TO_READ: usize = 1;
    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    result.resize_with(N_VALUES_TO_READ, Default::default);
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream::<TStore>(
        db_location,
        &mut stream,
        &point,
        N_VALUES_TO_READ as u64
    ).unwrap();

    stream.seek(SeekFrom::Start(0)).unwrap();

    unpack_stream(&mut stream, N_VALUES_TO_READ, &mut result);

    assert_eq!(result[0], value);
}

pub fn checked_write_unchecked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value: TStore::ValueType,
            point: [u64; 4],
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Copy + Debug + PartialEq + Default {
    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &AXIS
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TStore>(
        db_location,
        &point[0..3],
    ).unwrap();

    cql_db::write_value::<TStore>(
        db_location,
        &point,
        value
    ).unwrap();

    const N_VALUES_TO_READ: usize = 1;
    let mut result = Vec::with_capacity(N_VALUES_TO_READ);
    result.resize_with(N_VALUES_TO_READ, Default::default);
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream_unchecked::<TStore>(
        db_location,
        &mut stream,
        &point,
        N_VALUES_TO_READ as u64
    ).unwrap();

    stream.seek(SeekFrom::Start(0)).unwrap();

    unpack_stream(&mut stream, N_VALUES_TO_READ, &mut result);

    assert_eq!(result[0], value);
}
