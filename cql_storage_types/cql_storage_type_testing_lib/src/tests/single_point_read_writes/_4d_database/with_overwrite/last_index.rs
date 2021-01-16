use std::fmt::{ Debug };
use std::cmp::{ PartialEq };
use cql_model::{ CqlWritable, CqlReadable };
use crate::tests::single_point_read_writes::_4d_database::with_overwrite::test_functions;

const POINT: [u64; 4] = [3, 5, 7, 9];

pub fn unchecked<TStore: CqlWritable + CqlReadable>(db_location: &str, value1: TStore::ValueType, value2: TStore::ValueType)
    where TStore::ValueType: Clone + Debug + PartialEq {
        test_functions::unchecked::<TStore>(db_location, value1, value2, POINT)
}

pub fn checked<TStore: CqlWritable + CqlReadable>(db_location: &str, value1: TStore::ValueType, value2: TStore::ValueType)
    where TStore::ValueType: Clone + Debug + PartialEq {
        test_functions::checked::<TStore>(db_location, value1, value2, POINT)
}

pub fn unchecked_write_unchecked_overwrite_checked_read<TStore: CqlWritable + CqlReadable>(db_location: &str, value1: TStore::ValueType, value2: TStore::ValueType)
    where TStore::ValueType: Clone + Debug + PartialEq {
        test_functions::unchecked_write_unchecked_overwrite_checked_read::<TStore>(db_location, value1, value2, POINT)
}

pub fn unchecked_write_checked_overwrite_checked_read<TStore: CqlWritable + CqlReadable>(db_location: &str, value1: TStore::ValueType, value2: TStore::ValueType)
    where TStore::ValueType: Clone + Debug + PartialEq {
        test_functions::unchecked_write_checked_overwrite_checked_read::<TStore>(db_location, value1, value2, POINT)
}

pub fn checked_write_unchecked_overwrite_checked_read<TStore: CqlWritable + CqlReadable>(db_location: &str, value1: TStore::ValueType, value2: TStore::ValueType)
    where TStore::ValueType: Clone + Debug + PartialEq {
        test_functions::checked_write_unchecked_overwrite_checked_read::<TStore>(db_location, value1, value2, POINT)
}

pub fn checked_write_unchecked_overwrite_unchecked_read<TStore: CqlWritable + CqlReadable>(db_location: &str, value1: TStore::ValueType, value2: TStore::ValueType)
    where TStore::ValueType: Clone + Debug + PartialEq {
        test_functions::checked_write_unchecked_overwrite_unchecked_read::<TStore>(db_location, value1, value2, POINT)
}

pub fn unchecked_write_checked_overwrite_unchecked_read<TStore: CqlWritable + CqlReadable>(db_location: &str, value1: TStore::ValueType, value2: TStore::ValueType)
    where TStore::ValueType: Clone + Debug + PartialEq {
        test_functions::unchecked_write_checked_overwrite_unchecked_read::<TStore>(db_location, value1, value2, POINT)
}

pub fn checked_write_checked_overwrite_unchecked_read<TStore: CqlWritable + CqlReadable>(db_location: &str, value1: TStore::ValueType, value2: TStore::ValueType)
    where TStore::ValueType: Clone + Debug + PartialEq {
        test_functions::checked_write_checked_overwrite_unchecked_read::<TStore>(db_location, value1, value2, POINT)
}
