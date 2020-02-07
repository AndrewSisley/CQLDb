mod constants;

use std::io::{ Cursor, SeekFrom, Seek };
use serial_test::serial;
use cql_tiny_text::{ TinyText, unpack_stream };
use constants::DATABASE_LOCATION;

#[test]
#[serial]
fn _1d_tiny_text_database_allows_for_single_point_read_writes() {
    let axis = [
        cql_db::AxisDefinition {
            id: 1,
            max: 2,
        },
    ];

    let point1 = [2];
    let value1 = "test";

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &axis
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        value1.to_string()
    );

    let result1 = cql_db::read_value::<TinyText>(
        DATABASE_LOCATION,
        &point1
    );

    assert_eq!(result1, value1.to_string());
}

#[test]
#[serial]
fn _4d_tiny_text_database_allows_for_single_point_read_writes() {
    let axis = [
        cql_db::AxisDefinition {
            id: 1,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 2,
            max: 5,
        },
        cql_db::AxisDefinition {
            id: 3,
            max: 3,
        },
        cql_db::AxisDefinition {
            id: 4,
            max: 2
        },
    ];

    let point1 = [2, 4, 3, 1];
    let value1 = "test 1";

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &axis
    );

    let first_to_second_key = cql_db::add_key::<TinyText>(
        DATABASE_LOCATION,
        point1[0],
        point1[1],
        &axis[0],
        &axis[1]
    );

    cql_db::add_key::<TinyText>(
        DATABASE_LOCATION,
        first_to_second_key,
        point1[2],
        &axis[1],
        &axis[2]
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        value1.to_string()
    );

    let result1 = cql_db::read_value::<TinyText>(
        DATABASE_LOCATION,
        &point1
    );

    assert_eq!(result1, value1.to_string());
}

#[test]
#[serial]
fn _4d_tiny_text_database_allows_for_single_point_read_writes_given_multiple_values_and_overwrites() {
    let axis = [
        cql_db::AxisDefinition {
            id: 1,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 2,
            max: 5,
        },
        cql_db::AxisDefinition {
            id: 3,
            max: 3,
        },
        cql_db::AxisDefinition {
            id: 4,
            max: 4
        },
    ];

    let point1 = [2, 4, 3, 1];
    let point2 = [1, 4, 3, 1];
    let point3 = [2, 1, 3, 1];
    let point4 = [2, 4, 3, 2];
    let value1 = "Test 1";
    let value2 = "Test 2";
    let value3 = "Test 3";
    let value5 = "Test 5";

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &axis
    );

    let first_to_second_key1 = cql_db::add_key::<TinyText>(
        DATABASE_LOCATION,
        point1[0],
        point1[1],
        &axis[0],
        &axis[1]
    );

    let first_to_second_key2 = cql_db::add_key::<TinyText>(
        DATABASE_LOCATION,
        point2[0],
        point2[1],
        &axis[0],
        &axis[1]
    );

    let first_to_second_key3 = cql_db::add_key::<TinyText>(
        DATABASE_LOCATION,
        point3[0],
        point3[1],
        &axis[0],
        &axis[1]
    );

    cql_db::add_key::<TinyText>(
        DATABASE_LOCATION,
        first_to_second_key1,
        point1[2],
        &axis[1],
        &axis[2]
    );

    cql_db::add_key::<TinyText>(
        DATABASE_LOCATION,
        first_to_second_key2,
        point2[2],
        &axis[1],
        &axis[2]
    );

    cql_db::add_key::<TinyText>(
        DATABASE_LOCATION,
        first_to_second_key3,
        point3[2],
        &axis[1],
        &axis[2]
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        value1.to_string()
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point2,
        value2.to_string()
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point3,
        value3.to_string()
    );

    let result1 = cql_db::read_value::<TinyText>(
        DATABASE_LOCATION,
        &point1
    );

    let result2 = cql_db::read_value::<TinyText>(
        DATABASE_LOCATION,
        &point2
    );

    let result3 = cql_db::read_value::<TinyText>(
        DATABASE_LOCATION,
        &point3
    );

    let result4 = cql_db::read_value::<TinyText>(
        DATABASE_LOCATION,
        &point4
    );

    assert_eq!(result1, value1.to_string());
    assert_eq!(result2, value2.to_string());
    assert_eq!(result3, value3.to_string());
    assert_eq!(result4, String::new());

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point2,
        value5.to_string()
    );

    let result5 = cql_db::read_value::<TinyText>(
        DATABASE_LOCATION,
        &point2
    );

    assert_eq!(result5, value5.to_string());
}

#[test]
#[serial]
fn _1d_tiny_text_database_allows_for_single_point_populated_stream_reads() {
    let axis = [
        cql_db::AxisDefinition {
            id: 1,
            max: 2,
        },
    ];

    let n_values_to_read = 1;
    let point1 = [2];
    let value1 = "test";

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &axis
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        value1.to_string()
    );

    let mut result: [String; 1] = [String::new(); 1];
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream::<TinyText>(
        DATABASE_LOCATION,
        &mut stream,
        &point1,
        n_values_to_read as u64
    );

    stream.seek(SeekFrom::Start(0)).unwrap();

    unpack_stream(&mut stream, n_values_to_read, |idx, value| {
        result[idx] = value
    });

    assert_eq!(result[0], value1.to_string());
}

#[test]
#[serial]
fn _1d_tiny_text_database_allows_for_single_point_empty_stream_reads() {
    let axis = [
        cql_db::AxisDefinition {
            id: 1,
            max: 2,
        },
    ];

    let n_values_to_read = 1;
    let point1 = [2];

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &axis
    );

    let mut result: [String; 1] = [String::new(); 1];
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream::<TinyText>(
        DATABASE_LOCATION,
        &mut stream,
        &point1,
        n_values_to_read as u64
    );

    stream.seek(SeekFrom::Start(0)).unwrap();

    unpack_stream(&mut stream, n_values_to_read, |idx, value| {
        result[idx] = value
    });

    assert_eq!(result[0], String::new());
}

#[test]
#[serial]
fn _1d_tiny_text_database_allows_for_multi_point_stream_reads() {
    let axis = [
        cql_db::AxisDefinition {
            id: 1,
            max: 5,
        },
    ];

    const N_VALUES_TO_READ: usize = 5;
    let point1 = [1];
    let point2 = [2];
    let point4 = [4];
    let value1 = "test1";
    let value2 = "test2";
    let value4 = "test4";

    cql_db::create_db::<TinyText>(
        DATABASE_LOCATION,
        &axis
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point1,
        value1.to_string()
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point2,
        value2.to_string()
    );

    cql_db::write_value::<TinyText>(
        DATABASE_LOCATION,
        &point4,
        value4.to_string()
    );

    let mut result: [String; N_VALUES_TO_READ] = [
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    ];
    let mut stream = Cursor::new(Vec::new());

    cql_db::read_to_stream::<TinyText>(
        DATABASE_LOCATION,
        &mut stream,
        &point1,
        N_VALUES_TO_READ as u64
    );

    stream.seek(SeekFrom::Start(0)).unwrap();

    unpack_stream(&mut stream, N_VALUES_TO_READ, |idx, value| {
        result[idx] = value
    });

    assert_eq!(result[0], value1.to_string());
    assert_eq!(result[1], value2.to_string());
    assert_eq!(result[2], String::new());
    assert_eq!(result[3], value4.to_string());
    assert_eq!(result[4], String::new());
}
