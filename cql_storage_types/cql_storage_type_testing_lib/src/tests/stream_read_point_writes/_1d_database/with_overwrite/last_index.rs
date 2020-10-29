use std::fmt::{ Debug };
use std::cmp::{ PartialEq };
use std::io::{ Cursor };
use cql_model::{ CqlWritable, CqlStreamReadable };
use crate::tests::stream_read_point_writes::_1d_database::with_overwrite::test_functions;

const POINT: [u64; 1] = [3];

pub fn unchecked<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Copy + Debug + PartialEq + Default {
    test_functions::unchecked::<TStore>(db_location, value1, value2, POINT, unpack_stream)
}

pub fn checked<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Copy + Debug + PartialEq + Default {
    test_functions::checked::<TStore>(db_location, value1, value2, POINT, unpack_stream)
}

pub fn unchecked_write_unchecked_overwrite_checked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Copy + Debug + PartialEq + Default {
    test_functions::unchecked_write_unchecked_overwrite_checked_read::<TStore>(db_location, value1, value2, POINT, unpack_stream)
}

pub fn unchecked_write_checked_overwrite_checked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Copy + Debug + PartialEq + Default {
    test_functions::unchecked_write_checked_overwrite_checked_read::<TStore>(db_location, value1, value2, POINT, unpack_stream)
}

pub fn checked_write_unchecked_overwrite_checked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Copy + Debug + PartialEq + Default {
    test_functions::checked_write_unchecked_overwrite_checked_read::<TStore>(db_location, value1, value2, POINT, unpack_stream)
}

pub fn checked_write_unchecked_overwrite_unchecked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Copy + Debug + PartialEq + Default {
    test_functions::checked_write_unchecked_overwrite_unchecked_read::<TStore>(db_location, value1, value2, POINT, unpack_stream)
}

pub fn unchecked_write_checked_overwrite_unchecked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Copy + Debug + PartialEq + Default {
    test_functions::unchecked_write_checked_overwrite_unchecked_read::<TStore>(db_location, value1, value2, POINT, unpack_stream)
}

pub fn checked_write_checked_overwrite_unchecked_read<'a, TStore: CqlWritable + CqlStreamReadable>(
            db_location: &str,
            value1: TStore::ValueType,
            value2: TStore::ValueType,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        )
        where TStore::ValueType: Copy + Debug + PartialEq + Default {
    test_functions::checked_write_checked_overwrite_unchecked_read::<TStore>(db_location, value1, value2, POINT, unpack_stream)
}
