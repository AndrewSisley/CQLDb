mod constants;

use serial_test::serial;
use constants::DATABASE_LOCATION;

#[test]
#[serial]
fn _1d_f64_nullable_database_allows_for_single_point_read_writes() {
    let axis = [
        cql_db::AxisDefinition {
            id: 1,
            max: 2,
        },
    ];

    let point1 = [2];
    let value1 = Some(42.87);

    cql_db::create_db::<Option<f64>>(
        DATABASE_LOCATION,
        &axis
    );

    cql_db::write_value::<Option<f64>>(
        DATABASE_LOCATION,
        &point1,
        value1
    );

    let result1 = cql_db::read_value::<Option<f64>>(
        DATABASE_LOCATION,
        &point1
    );

    assert_eq!(result1, value1);
}

#[test]
#[serial]
fn _4d_f64_nullable_database_allows_for_single_point_read_writes() {
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
    let value1 = Some(-5.6);

    cql_db::create_db::<Option<f64>>(
        DATABASE_LOCATION,
        &axis
    );

    let first_to_second_key = cql_db::add_key::<Option<f64>>(
        DATABASE_LOCATION,
        point1[0],
        point1[1],
        &axis[0],
        &axis[1]
    );

    cql_db::add_key::<Option<f64>>(
        DATABASE_LOCATION,
        first_to_second_key,
        point1[2],
        &axis[1],
        &axis[2]
    );

    cql_db::write_value::<Option<f64>>(
        DATABASE_LOCATION,
        &point1,
        value1
    );

    let result1 = cql_db::read_value::<Option<f64>>(
        DATABASE_LOCATION,
        &point1
    );

    assert_eq!(result1, value1);
}
