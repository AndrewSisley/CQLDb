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
            2,
        ]
    )
}

#[test]
fn create_db__creates_db__given_3D_definition() {
    cql_db::create_db::<DummyType>(
        DATABASE_LOCATION,
        &[
            2,
            1,
            3,
        ]
    )
}

struct DummyType;

impl CqlType for DummyType {
    type ValueType = Option<f64>;
    const VALUE_SIZE: usize = 9;
}
