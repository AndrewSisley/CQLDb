mod constants;

use std::io::{ Cursor, SeekFrom, Seek };
use serial_test::serial;
use std::convert::TryFrom;
use cql_tiny_text::{ TinyText, unpack_stream };
use constants::DATABASE_LOCATION;

#[test]
#[serial]
fn _1d_tiny_text_database_allows_for_single_point_read_writes() {
    let axis = [
        2,
    ];

    let point1 = [2];
    let value1 = "test";

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &axis
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1
    ).unwrap();

    assert_eq!(String::from(result1), value1);
}

#[test]
#[serial]
fn _1d_tiny_text_database_allows_for_single_point_255_char_read_writes() {
    let point1 = [1];
    let value1 = "1";

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &[1]
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1.repeat(255)).unwrap()
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1
    ).unwrap();

    assert_eq!(String::from(result1), value1.repeat(255));
}

#[test]
#[serial]
fn _1d_tiny_text_database_allows_for_single_point_empty_read_writes() {
    let point1 = [1];

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &[1]
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::new()
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1
    ).unwrap();

    assert_eq!(String::from(result1), String::new());
}

#[test]
#[serial]
fn _4d_tiny_text_database_allows_for_single_point_read_writes() {
    let axis = [
        2,
        5,
        3,
        2,
    ];

    let point1 = [2, 4, 3, 1];
    let value1 = "test 1";

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &axis
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1[0..3],
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1
    ).unwrap();

    assert_eq!(String::from(result1), value1);
}

#[test]
#[serial]
fn _4d_tiny_text_database_allows_for_single_point_read_writes_given_multiple_values_and_overwrites() {
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
    let value1 = "Test 1";
    let value2 = "Test 2";
    let value3 = "Test 3";
    let value5 = "Test 5";

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &axis
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1[0..3],
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point2[0..3],
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point3[0..3],
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point2,
        TinyText::try_from(value2).unwrap()
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point3,
        TinyText::try_from(value3).unwrap()
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1
    ).unwrap();

    let result2 = cql_db::read_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point2
    ).unwrap();

    let result3 = cql_db::read_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point3
    ).unwrap();

    let result4 = cql_db::read_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point4
    ).unwrap();

    assert_eq!(String::from(result1), value1);
    assert_eq!(String::from(result2), value2);
    assert_eq!(String::from(result3), value3);
    assert_eq!(String::from(result4), String::new());

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point2,
        TinyText::try_from(value5).unwrap()
    ).unwrap();

    let result5 = cql_db::read_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point2
    ).unwrap();

    assert_eq!(String::from(result5), value5);
}

#[test]
#[serial]
fn _1d_tiny_text_database_allows_for_single_point_populated_stream_reads() {
    let axis = [
        2,
    ];

    let n_values_to_read = 1;
    let point1 = [2];
    let value1 = "test";

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &axis
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    ).unwrap();

    let mut result: [TinyText; 1] = [TinyText::new(); 1];
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &mut stream,
        &point1,
        n_values_to_read as u64
    ).unwrap();

    stream.seek(SeekFrom::Start(0)).unwrap();

    unpack_stream(&mut stream, n_values_to_read, |idx, value| {
        result[idx] = value
    });

    assert_eq!(String::from(result[0].clone()), value1);
}

#[test]
#[serial]
fn _1d_tiny_text_database_allows_for_single_point_empty_stream_reads() {
    let axis = [
        2,
    ];

    let n_values_to_read = 1;
    let point1 = [2];

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &axis
    ).unwrap();

    let mut result: [TinyText; 1] = [TinyText::new(); 1];
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &mut stream,
        &point1,
        n_values_to_read as u64
    ).unwrap();

    stream.seek(SeekFrom::Start(0)).unwrap();

    unpack_stream(&mut stream, n_values_to_read, |idx, value| {
        result[idx] = value
    });

    assert_eq!(String::from(result[0].clone()), String::new());
}

#[test]
#[serial]
fn _1d_tiny_text_database_allows_for_multi_point_stream_reads() {
    let axis = [
        5,
    ];

    const N_VALUES_TO_READ: usize = 5;
    let point1 = [1];
    let point2 = [2];
    let point4 = [4];
    let value1 = "test1";
    let value2 = "test2";
    let value4 = "test4";

    cql_db::create_db_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &axis
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        TinyText::try_from(value1).unwrap()
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point2,
        TinyText::try_from(value2).unwrap()
    ).unwrap();

    cql_db::write_value_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &point4,
        TinyText::try_from(value4).unwrap()
    ).unwrap();

    let mut result: [TinyText; N_VALUES_TO_READ] = [
        TinyText::new(),
        TinyText::new(),
        TinyText::new(),
        TinyText::new(),
        TinyText::new(),
    ];
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream_unchecked::<TinyText>(
        DATABASE_LOCATION,
        &mut stream,
        &point1,
        N_VALUES_TO_READ as u64
    ).unwrap();

    stream.seek(SeekFrom::Start(0)).unwrap();

    unpack_stream(&mut stream, N_VALUES_TO_READ, |idx, value| {
        result[idx] = value
    });

    assert_eq!(String::from(result[0].clone()), value1);
    assert_eq!(String::from(result[1].clone()), value2);
    assert_eq!(String::from(result[2].clone()), String::new());
    assert_eq!(String::from(result[3].clone()), value4);
    assert_eq!(String::from(result[4].clone()), String::new());
}
