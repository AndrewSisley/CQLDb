#![allow(non_snake_case)]

mod constants;

use serial_test::serial;
use std::fs::remove_file;

use constants::{ DATABASE_LOCATION };
use cql_model::{ CqlType };
use cql_db::error;

const DB_FILE_NAME: &str = "/db";
const AXIS_FILE_NAME: &str = "/ax";
const KEY_FILE_NAME: &str = "/key";

#[test]
#[serial]
#[should_panic]
fn create_db_unchecked__panics__given_0D_definition() {
    cql_db::create_db_unchecked::<DummyType>(
        DATABASE_LOCATION,
        &[]
    ).unwrap()
}

#[test]
#[serial]
fn create_db__returns_InsufficientDimensionsError__given_0D_definition() {
    let db_dimensions = [];

    let result = match cql_db::create_db::<DummyType>(
        DATABASE_LOCATION,
        &db_dimensions
    ) {
        Err(error) => match error {
            error::Error::Cql(cql_error) => match cql_error {
                error::cql::Error::DimensionsOutOfRangeError{ requested, min, max } => Some((requested, min, max)),
                _ => None,
            },
            _ => None,
        }
        _ => None,
    };

    assert_eq!(
        result.unwrap(),
        (db_dimensions.len(), 1, u64::max_value() as usize - 1)
    );
}

#[test]
#[serial]
fn create_db__returns_DimensionTooSmallError__given_1D_definition_with_capacity_0() {
    let result = match cql_db::create_db::<DummyType>(
        DATABASE_LOCATION,
        &[0]
    ) {
        Err(error) => match error {
            error::Error::Cql(cql_error) => Some(cql_error),
            _ => None,
        }
        _ => None,
    };

    assert_eq!(
        result.unwrap(),
        error::cql::Error::DimensionTooSmallError
    );
}

#[test]
#[serial]
fn create_db_unchecked__creates_db__given_1D_definition() {
    cql_db::create_db_unchecked::<DummyType>(
        DATABASE_LOCATION,
        &[2]
    ).unwrap()
}

#[test]
#[serial]
fn create_db__creates_db__given_1D_definition() {
    delete_existing_db();

    cql_db::create_db::<DummyType>(
        DATABASE_LOCATION,
        &[2]
    ).unwrap()
}

#[test]
#[serial]
fn create_db__returns_I0_AlreadyExists__given_1D_definition_and_db_already_exists() {
    delete_existing_db();

    cql_db::create_db::<DummyType>(
        DATABASE_LOCATION,
        &[2]
    ).unwrap();

    let result = match cql_db::create_db::<DummyType>(
        DATABASE_LOCATION,
        &[2]
    ) {
        Err(error) => match error {
            error::Error::Io(io_error) => Some(io_error),
            _ => None,
        }
        _ => None,
    };

    assert_eq!(
        result.unwrap().kind(),
        std::io::ErrorKind::AlreadyExists
    );
}

#[test]
#[serial]
fn create_db__returns_DimensionTooSmallError__given_3D_definition_with_capacity_0_1_1() {
    let result = match cql_db::create_db::<DummyType>(
        DATABASE_LOCATION,
        &[0, 1, 1]
    ) {
        Err(error) => match error {
            error::Error::Cql(cql_error) => Some(cql_error),
            _ => None,
        }
        _ => None,
    };

    assert_eq!(
        result.unwrap(),
        error::cql::Error::DimensionTooSmallError
    );
}

#[test]
#[serial]
fn create_db__returns_DimensionTooSmallError__given_3D_definition_with_capacity_1_0_1() {
    let result = match cql_db::create_db::<DummyType>(
        DATABASE_LOCATION,
        &[1, 0, 1]
    ) {
        Err(error) => match error {
            error::Error::Cql(cql_error) => Some(cql_error),
            _ => None,
        }
        _ => None,
    };

    assert_eq!(
        result.unwrap(),
        error::cql::Error::DimensionTooSmallError
    );
}

#[test]
#[serial]
fn create_db__returns_DimensionTooSmallError__given_3D_definition_with_capacity_1_1_0() {
    let result = match cql_db::create_db::<DummyType>(
        DATABASE_LOCATION,
        &[1, 1, 0]
    ) {
        Err(error) => match error {
            error::Error::Cql(cql_error) => Some(cql_error),
            _ => None,
        }
        _ => None,
    };

    assert_eq!(
        result.unwrap(),
        error::cql::Error::DimensionTooSmallError
    );
}

#[test]
#[serial]
fn create_db_unchecked__creates_db__given_3D_definition() {
    cql_db::create_db_unchecked::<DummyType>(
        DATABASE_LOCATION,
        &[
            1,
            1,
            1,
        ]
    ).unwrap()
}

#[test]
#[serial]
fn create_db__creates_db__given_3D_definition() {
    delete_existing_db();
    let _ = remove_file(format!("{}{}{}_{}", DATABASE_LOCATION, KEY_FILE_NAME, 1, 2));

    cql_db::create_db::<DummyType>(
        DATABASE_LOCATION,
        &[1, 1, 1]
    ).unwrap()
}

#[test]
#[serial]
fn create_db__returns_I0_AlreadyExists__given_3D_definition_and_db_already_exists() {
    delete_existing_db();
    let _ = remove_file(format!("{}{}{}_{}", DATABASE_LOCATION, KEY_FILE_NAME, 1, 2));

    cql_db::create_db::<DummyType>(
        DATABASE_LOCATION,
        &[1, 1, 1]
    ).unwrap();

    let result = match cql_db::create_db::<DummyType>(
        DATABASE_LOCATION,
        &[1, 1, 1]
    ) {
        Err(error) => match error {
            error::Error::Io(io_error) => Some(io_error),
            _ => None,
        }
        _ => None,
    };

    assert_eq!(
        result.unwrap().kind(),
        std::io::ErrorKind::AlreadyExists
    );
}

fn delete_existing_db() {
    let _ = remove_file(format!("{}{}", DATABASE_LOCATION, DB_FILE_NAME));
    let _ = remove_file(format!("{}{}", DATABASE_LOCATION, AXIS_FILE_NAME));
}

struct DummyType;

impl CqlType for DummyType {
    type ValueType = Option<f64>;
    const VALUE_SIZE: usize = 9;
}
