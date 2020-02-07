#![allow(non_snake_case)]

mod constants;

use constants::DATABASE_LOCATION;
use cql_model::{ CqlType };

#[test]
#[should_panic]
fn create_db__panics__given_0D_definition() {
    cql_db::create_db::<DummyType>(
        DATABASE_LOCATION,
        &[]
    )
}

#[test]
fn create_db__creates_db__given_1D_definition() {
    cql_db::create_db::<DummyType>(
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
fn create_db__creates_db__given_3D_definition() {
    cql_db::create_db::<DummyType>(
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

struct DummyType;

impl CqlType for DummyType {
    type ValueType = Option<f64>;

    fn grow_database(_db_location: &str, _size_to_grow: u64) {
        panic!("Not implemented")
    }
}
