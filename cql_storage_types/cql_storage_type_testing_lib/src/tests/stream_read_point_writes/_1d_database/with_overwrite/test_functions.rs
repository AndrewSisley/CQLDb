use std::fmt::{ Debug };
use std::cmp::{ PartialEq };
use std::io::{ Cursor, SeekFrom, Seek };
use cql_model::{ CqlWritable, CqlStreamReadable };

const AXIS: [u64; 1] = [
    3,
];

pub fn unchecked<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            point: [u64; 1],
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Clone + Debug + PartialEq + Default {
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

    assert_eq!(result[0], value2);
}

pub fn checked<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            point: [u64; 1],
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Clone + Debug + PartialEq + Default {
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

    assert_eq!(result[0], value2);
}

pub fn unchecked_write_unchecked_overwrite_checked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            point: [u64; 1],
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Clone + Debug + PartialEq + Default {
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

    assert_eq!(result[0], value2);
}

pub fn unchecked_write_checked_overwrite_checked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            point: [u64; 1],
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Clone + Debug + PartialEq + Default {
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

    assert_eq!(result[0], value2);
}

pub fn checked_write_unchecked_overwrite_checked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            point: [u64; 1],
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Clone + Debug + PartialEq + Default {
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

    assert_eq!(result[0], value2);
}

pub fn checked_write_unchecked_overwrite_unchecked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            point: [u64; 1],
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Clone + Debug + PartialEq + Default {
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

    assert_eq!(result[0], value2);
}

pub fn unchecked_write_checked_overwrite_unchecked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            point: [u64; 1],
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Clone + Debug + PartialEq + Default {
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

    assert_eq!(result[0], value2);
}

pub fn checked_write_checked_overwrite_unchecked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            point: [u64; 1],
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Clone + Debug + PartialEq + Default {
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

    assert_eq!(result[0], value2);
}
