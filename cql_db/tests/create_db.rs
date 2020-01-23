#![allow(non_snake_case)]

mod constants;

use constants::DATABASE_LOCATION;

#[test]
#[should_panic]
fn create_db__panics__given_0D_definition_and_option_f64() {
    cql_db::create_db::<Option<f64>>(
        DATABASE_LOCATION,
        &[]
    )
}

#[test]
#[should_panic]
fn create_db__panics__given_0D_definition_and_tiny_text() {
    cql_db::create_db::<cql_storage::tiny_text::TinyText>(
        DATABASE_LOCATION,
        &[]
    )
}

#[test]
fn create_db__creates_db__given_1D_definition_and_option_f64() {
    cql_db::create_db::<Option<f64>>(
        DATABASE_LOCATION,
        &[
            cql_db::AxisDefinition {
                id: 1,
                max: 2,
            },
        ]
    )
}

#[test]
fn create_db__creates_db__given_1D_definition_and_tiny_text() {
    cql_db::create_db::<cql_storage::tiny_text::TinyText>(
        DATABASE_LOCATION,
        &[
            cql_db::AxisDefinition {
                id: 1,
                max: 2,
            },
        ]
    )
}

#[test]
fn create_db__creates_db__given_3D_definition_and_option_f64() {
    cql_db::create_db::<Option<f64>>(
        DATABASE_LOCATION,
        &[
            cql_db::AxisDefinition {
                id: 1,
                max: 2,
            },
            cql_db::AxisDefinition {
                id: 2,
                max: 1,
            },
            cql_db::AxisDefinition {
                id: 3,
                max: 3,
            },
        ]
    )
}

#[test]
fn create_db__creates_db__given_3D_definition_and_tiny_text() {
    cql_db::create_db::<cql_storage::tiny_text::TinyText>(
        DATABASE_LOCATION,
        &[
            cql_db::AxisDefinition {
                id: 1,
                max: 2,
            },
            cql_db::AxisDefinition {
                id: 2,
                max: 1,
            },
            cql_db::AxisDefinition {
                id: 3,
                max: 3,
            },
        ]
    )
}
