#![allow(non_snake_case)]

mod constants;

use serial_test::serial;
use std::fs::OpenOptions;

use constants::DATABASE_LOCATION;
use cql_model::CqlType;
use cql_u64::{ U64 };
use cql_db::error;

#[test]
#[serial]
fn write_value__returns_DimensionsOutOfRangeError__given_1d_u64_database_and_0d_location() {
    let db_dimensions = [1];

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    ).unwrap();

    let location = [];

    let result = match cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &location,
        42
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
        (location.len(), db_dimensions.len(), db_dimensions.len())
    );
}

#[test]
#[serial]
fn write_value__returns_DimensionsOutOfRangeError__given_1d_u64_database_and_2d_location() {
    let db_dimensions = [1];

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    ).unwrap();

    let location = [1, 1];

    let result = match cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &location,
        42
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
        (location.len(), db_dimensions.len(), db_dimensions.len())
    );
}

#[test]
#[serial]
fn write_value__returns_DimensionsOutOfRangeError__given_2d_u64_database_and_1d_location() {
    let db_dimensions = [1, 1];

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    ).unwrap();

    let location = [1];

    let result = match cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &location,
        42
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
        (location.len(), db_dimensions.len(), db_dimensions.len())
    );
}

#[test]
#[serial]
fn write_value__returns_DimensionsOutOfRangeError__given_2d_u64_database_and_3d_location() {
    let db_dimensions = [1, 1];

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    ).unwrap();

    let location = [1, 1, 1];

    let result = match cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &location,
        42
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
        (location.len(), db_dimensions.len(), db_dimensions.len())
    );
}

#[test]
#[serial]
#[should_panic]
fn _1d_u64_database_panics_given_zero_index() {
    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[1]
    ).unwrap();

    cql_db::write_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &[0],
        42
    ).unwrap();
}

#[test]
#[serial]
fn write_value__returns_IndexOutOfRangeError__given_1d_u64_database_and_zero_index() {
    let db_dimensions = [1];

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    ).unwrap();

    let small_index = 0;
    let location = [0];

    let result = match cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &location,
        42
    ) {
        Err(error) => match error {
            error::Error::Cql(cql_error) => match cql_error {
                error::cql::Error::IndexOutOfRangeError{ dimension_index, requested, min, max } => Some((dimension_index, requested, min, max)),
                _ => None,
            },
            _ => None,
        }
        _ => None,
    };

    assert_eq!(
        result.unwrap(),
        (small_index, location[small_index], 1, db_dimensions[small_index])
    );
}

#[test]
#[serial]
#[should_panic]
fn _2d_u64_database_panics_given_one_zero_index() {
    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[1, 1]
    ).unwrap();

    cql_db::write_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &[1, 0],
        42
    ).unwrap();
}

#[test]
#[serial]
fn write_value__returns_IndexOutOfRangeError__given_2d_u64_database_and_one_zero_index() {
    let db_dimensions = [1, 1];

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    ).unwrap();

    let small_index = 1;
    let location = [1, 0];

    let result = match cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &location,
        42
    ) {
        Err(error) => match error {
            error::Error::Cql(cql_error) => match cql_error {
                error::cql::Error::IndexOutOfRangeError{ dimension_index, requested, min, max } => Some((dimension_index, requested, min, max)),
                _ => None,
            },
            _ => None,
        }
        _ => None,
    };

    assert_eq!(
        result.unwrap(),
        (small_index, location[small_index], 1, db_dimensions[small_index])
    );
}

#[test]
#[serial]
#[should_panic]
fn _2d_u64_database_panics_given_zero_one_index() {
    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[1, 1]
    ).unwrap();

    cql_db::write_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &[0, 1],
        42
    ).unwrap();
}

#[test]
#[serial]
fn write_value__returns_IndexOutOfRangeError__given_2d_u64_database_and_zero_one_index() {
    let db_dimensions = [1, 1];

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    ).unwrap();

    let small_index = 0;
    let location = [0, 1];

    let result = match cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &location,
        42
    ) {
        Err(error) => match error {
            error::Error::Cql(cql_error) => match cql_error {
                error::cql::Error::IndexOutOfRangeError{ dimension_index, requested, min, max } => Some((dimension_index, requested, min, max)),
                _ => None,
            },
            _ => None,
        }
        _ => None,
    };

    assert_eq!(
        result.unwrap(),
        (small_index, location[small_index], 1, db_dimensions[small_index])
    );
}

#[test]
#[serial]
fn _1d_u64_database_allows_for_first_item_to_be_written() {
    let point1 = [1];
    let value1 = 42;

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[1]
    ).unwrap();

    cql_db::write_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &point1,
        value1
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &point1
    ).unwrap();

    assert_eq!(result1, value1);
}

#[test]
#[serial]
fn _1d_u64_database_allows_for_first_item_to_be_written_checked() {
    let point1 = [1];
    let value1 = 42;

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[1]
    ).unwrap();

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &point1,
        value1
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &point1
    ).unwrap();

    assert_eq!(result1, value1);
}

#[test]
#[serial]
fn _2d_u64_database_allows_for_first_item_to_be_written() {
    let point1 = [1, 1];
    let value1 = 42;

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[1, 1]
    ).unwrap();

    cql_db::write_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &point1,
        value1
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &point1
    ).unwrap();

    assert_eq!(result1, value1);
}

#[test]
#[serial]
fn _2d_u64_database_allows_for_first_item_to_be_written_checked() {
    let point1 = [1, 1];
    let value1 = 42;

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[1, 1]
    ).unwrap();

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &point1,
        value1
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &point1
    ).unwrap();

    assert_eq!(result1, value1);
}

#[test]
#[serial]
#[should_panic]
fn _3d_u64_database_panics_writing_first_item_without_link() {
    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[1, 1, 1]
    ).unwrap();

    cql_db::write_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &[1, 1, 1],
        42
    ).unwrap();
}

#[test]
#[serial]
fn write_value__returns_ElementsNotLinkedError__given_3d_u64_database_and_elements_not_linked() {
    let db_dimensions = [1, 1, 1];

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    ).unwrap();

    let location = [1, 1, 1];

    let result = match cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &location,
        42
    ) {
        Err(error) => match error {
            error::Error::Cql(cql_error) => match cql_error {
                error::cql::Error::ElementsNotLinkedError{ x_dimension, x, y_dimension, y } => Some((x_dimension, x, y_dimension, y)),
                _ => None,
            },
            _ => None,
        }
        _ => None,
    };

    assert_eq!(
        result.unwrap(),
        (0, location[0], 1, location[1])
    );
}

#[test]
#[serial]
fn _3d_u64_database_allows_for_first_item_to_be_written_after_axis_linked() {
    let point1 = [1, 1, 1];
    let value1 = 42;

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[5, 5, 5]
    ).unwrap();

    cql_db::link_dimensions::<U64>(
        DATABASE_LOCATION,
        &point1[0..2]
    ).unwrap();

    cql_db::write_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &point1,
        value1
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &point1
    ).unwrap();

    assert_eq!(result1, value1);
}

#[test]
#[serial]
fn _3d_u64_database_allows_for_first_item_to_be_written_after_axis_linked_checked() {
    let point1 = [1, 1, 1];
    let value1 = 42;

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[5, 5, 5]
    ).unwrap();

    cql_db::link_dimensions::<U64>(
        DATABASE_LOCATION,
        &point1[0..2]
    ).unwrap();

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &point1,
        value1
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &point1
    ).unwrap();

    assert_eq!(result1, value1);
}

#[test]
#[serial]
fn _3d_u64_database_allows_for_last_item_to_be_written_after_axis_linked() {
    let point1 = [5, 5, 5];
    let value1 = 42;

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[5, 5, 5]
    ).unwrap();

    cql_db::link_dimensions::<U64>(
        DATABASE_LOCATION,
        &point1[0..2]
    ).unwrap();

    cql_db::write_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &point1,
        value1
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &point1
    ).unwrap();

    assert_eq!(result1, value1);
}

#[test]
#[serial]
fn _3d_u64_database_allows_for_last_item_to_be_written_after_axis_linked_checked() {
    let point1 = [5, 5, 5];
    let value1 = 42;

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[5, 5, 5]
    ).unwrap();

    cql_db::link_dimensions::<U64>(
        DATABASE_LOCATION,
        &point1[0..2]
    ).unwrap();

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &point1,
        value1
    ).unwrap();

    let result1 = cql_db::read_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &point1
    ).unwrap();

    assert_eq!(result1, value1);
}

#[test]
#[serial]
fn _1d_u64_database_expands_size_as_items_written() {
    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[5]
    ).unwrap();

    let initial_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));
    assert_eq!(initial_size, 0);

    cql_db::write_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &[1],
        42
    ).unwrap();

    let post_write_one_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));
    assert_eq!(post_write_one_size, 1 * U64::VALUE_SIZE as u64);

    cql_db::write_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &[3],
        42
    ).unwrap();

    let post_write_three_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));
    assert_eq!(post_write_three_size, 3 * U64::VALUE_SIZE as u64);
}

#[test]
#[serial]
fn _1d_u64_database_expands_size_as_items_written_checked() {
    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[5]
    ).unwrap();

    let initial_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));
    assert_eq!(initial_size, 0);

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &[1],
        42
    ).unwrap();

    let post_write_one_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));
    assert_eq!(post_write_one_size, 1 * U64::VALUE_SIZE as u64);

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &[3],
        42
    ).unwrap();

    let post_write_three_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));
    assert_eq!(post_write_three_size, 3 * U64::VALUE_SIZE as u64);
}

#[test]
#[serial]
fn _1d_u64_database_maintains_size_as_first_item_written_after_last_item_written() {
    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[5]
    ).unwrap();

    cql_db::write_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &[5],
        42
    ).unwrap();

    let post_write_to_last_point_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    cql_db::write_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &[1],
        42
    ).unwrap();

    let post_write_to_first_point_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    assert_eq!(post_write_to_first_point_size, post_write_to_last_point_size);
}

#[test]
#[serial]
fn _1d_u64_database_maintains_size_as_first_item_written_after_last_item_written_checked() {
    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[5]
    ).unwrap();

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &[5],
        42
    ).unwrap();

    let post_write_to_last_point_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &[1],
        42
    ).unwrap();

    let post_write_to_first_point_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    assert_eq!(post_write_to_first_point_size, post_write_to_last_point_size);
}

#[test]
#[serial]
fn _2d_u64_database_maintains_size_as_first_item_written_after_last_item_written() {
    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[5, 5]
    ).unwrap();

    cql_db::write_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &[5, 5],
        42
    ).unwrap();

    let post_write_to_last_point_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    cql_db::write_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &[1, 1],
        42
    ).unwrap();

    let post_write_to_first_point_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    assert_eq!(post_write_to_first_point_size, post_write_to_last_point_size);
}

#[test]
#[serial]
fn _2d_u64_database_maintains_size_as_first_item_written_after_last_item_written_checked() {
    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[5, 5]
    ).unwrap();

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &[5, 5],
        42
    ).unwrap();

    let post_write_to_last_point_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &[1, 1],
        42
    ).unwrap();

    let post_write_to_first_point_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    assert_eq!(post_write_to_first_point_size, post_write_to_last_point_size);
}

#[test]
#[serial]
fn _3d_u64_database_maintains_size_as_last_item_written_after_axis_linked() {
    let point1 = [5, 5, 5];

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[5, 5, 5]
    ).unwrap();

    cql_db::link_dimensions::<U64>(
        DATABASE_LOCATION,
        &point1[0..2]
    ).unwrap();

    let post_link_db_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    cql_db::write_value_unchecked::<U64>(
        DATABASE_LOCATION,
        &point1,
        42
    ).unwrap();

    let post_write_db_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    assert_eq!(post_link_db_size, post_write_db_size);
}

#[test]
#[serial]
fn _3d_u64_database_maintains_size_as_last_item_written_after_axis_linked_checked() {
    let point1 = [5, 5, 5];

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[5, 5, 5]
    ).unwrap();

    cql_db::link_dimensions::<U64>(
        DATABASE_LOCATION,
        &point1[0..2]
    ).unwrap();

    let post_link_db_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &point1,
        42
    ).unwrap();

    let post_write_db_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    assert_eq!(post_link_db_size, post_write_db_size);
}

fn get_file_length(file_path: &str) -> u64 {
    OpenOptions::new().read(true).open(file_path).unwrap().metadata().unwrap().len()
}
