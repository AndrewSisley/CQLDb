mod constants;

use serial_test::serial;
use std::fs::OpenOptions;

use constants::DATABASE_LOCATION;
use cql_model::CqlType;
use cql_u64::{ U64 };

#[test]
#[serial]
#[should_panic]
fn _1d_u64_database_panics_given_one_zero_index() {
    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[1]
    ).unwrap();

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &[0],
        42
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

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &[1, 0],
        42
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

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &[0, 1],
        42
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
fn _2d_u64_database_allows_for_first_item_to_be_written() {
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
    );

    let result1 = cql_db::read_value::<U64>(
        DATABASE_LOCATION,
        &point1
    );

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

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &[1, 1, 1],
        42
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

    cql_db::link_dimensions_unchecked::<U64>(
        DATABASE_LOCATION,
        &point1[0..3]
    ).unwrap();

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
fn _3d_u64_database_allows_for_last_item_to_be_written_after_axis_linked() {
    let point1 = [5, 5, 5];
    let value1 = 42;

    cql_db::create_db_unchecked::<U64>(
        DATABASE_LOCATION,
        &[5, 5, 5]
    ).unwrap();

    cql_db::link_dimensions_unchecked::<U64>(
        DATABASE_LOCATION,
        &point1[0..3]
    ).unwrap();

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
fn _1d_u64_database_expands_size_as_items_written() {
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
    );

    let post_write_one_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));
    assert_eq!(post_write_one_size, 1 * U64::VALUE_SIZE as u64);

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &[3],
        42
    );

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

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &[5],
        42
    );

    let post_write_to_last_point_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &[1],
        42
    );

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

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &[5, 5],
        42
    );

    let post_write_to_last_point_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &[1, 1],
        42
    );

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

    cql_db::link_dimensions_unchecked::<U64>(
        DATABASE_LOCATION,
        &point1[0..3]
    ).unwrap();

    let post_link_db_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    cql_db::write_value::<U64>(
        DATABASE_LOCATION,
        &point1,
        42
    );

    let post_write_db_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    assert_eq!(post_link_db_size, post_write_db_size);
}

fn get_file_length(file_path: &str) -> u64 {
    OpenOptions::new().read(true).open(file_path).unwrap().metadata().unwrap().len()
}
