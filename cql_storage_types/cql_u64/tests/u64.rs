mod constants;

use serial_test::serial;
use constants::DATABASE_LOCATION;
use cql_u64::U64;

#[test]
#[serial]
fn _1d_u64_database_allows_for_single_point_read_writes() {
    let axis = [
        2,
    ];

    let point1 = [2];
    let value1 = 42;

    cql_db::create_db::<U64>(
        DATABASE_LOCATION,
        &axis
    );

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &point1,
        value1
    );

    let result1 = cql_db::read_value::<U64>(
        DATABASE_LOCATION,
        &point1
    );

    assert_eq!(result1, value1);
}

#[test]
#[serial]
fn _4d_u64_database_allows_for_single_point_read_writes() {
    let axis = [
        2,
        5,
        3,
        2,
    ];

    let point1 = [2, 4, 3, 1];
    let value1 = 5;

    cql_db::create_db::<U64>(
        DATABASE_LOCATION,
        &axis
    );

    cql_db::link_dimensions::<U64>(
        DATABASE_LOCATION,
        &point1[0..3],
    );

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &point1,
        value1
    );

    let result1 = cql_db::read_value::<U64>(
        DATABASE_LOCATION,
        &point1
    );

    assert_eq!(result1, value1);
}

#[test]
#[serial]
fn _4d_u64_database_allows_for_single_point_read_writes_given_multiple_values_and_overwrites() {
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
    let value1 = 5;
    let value2 = 20;
    let value3 = 0;
    let value5 = 9999999999999;

    cql_db::create_db::<U64>(
        DATABASE_LOCATION,
        &axis
    );

    cql_db::link_dimensions::<U64>(
        DATABASE_LOCATION,
        &point1[0..3]
    );

    cql_db::link_dimensions::<U64>(
        DATABASE_LOCATION,
        &point2[0..3]
    );

    cql_db::link_dimensions::<U64>(
        DATABASE_LOCATION,
        &point3[0..3]
    );

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &point1,
        value1
    );

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &point2,
        value2
    );

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &point3,
        value3
    );

    let result1 = cql_db::read_value::<U64>(
        DATABASE_LOCATION,
        &point1
    );

    let result2 = cql_db::read_value::<U64>(
        DATABASE_LOCATION,
        &point2
    );

    let result3 = cql_db::read_value::<U64>(
        DATABASE_LOCATION,
        &point3
    );

    let result4 = cql_db::read_value::<U64>(
        DATABASE_LOCATION,
        &point4
    );

    assert_eq!(result1, value1);
    assert_eq!(result2, value2);
    assert_eq!(result3, value3);
    assert_eq!(result4, 0);

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &point2,
        value5
    );

    let result5 = cql_db::read_value::<U64>(
        DATABASE_LOCATION,
        &point2
    );

    assert_eq!(result5, value5);
}
