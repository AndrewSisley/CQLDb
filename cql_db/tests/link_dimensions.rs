mod constants;

use serial_test::serial;
use std::fs::OpenOptions;

use constants::DATABASE_LOCATION;
use cql_model::CqlType;
use cql_u64::{ U64 };

#[test]
#[serial]
fn _4d_u64_database_allows_for_first_dimension_pair_mins_to_be_linked() {
    let db_dimensions = [2, 3, 2, 2];

    cql_db::create_db::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    );

    cql_db::link_dimensions::<U64>(
        DATABASE_LOCATION,
        &[1, 1],
    );

    let key_file_size = get_file_length(&format!("{}/key1_2", DATABASE_LOCATION));
    let db_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    assert_eq!((1 + 1) * U64::VALUE_SIZE as u64, key_file_size);
    assert_eq!(0, db_size);
}

#[test]
#[serial]
fn _4d_u64_database_allows_for_first_dimension_pair_maxes_to_be_linked() {
    let db_dimensions = [2, 3, 2, 2];

    cql_db::create_db::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    );

    cql_db::link_dimensions::<U64>(
        DATABASE_LOCATION,
        &[2, 3],
    );

    let key_file_size = get_file_length(&format!("{}/key1_2", DATABASE_LOCATION));
    let db_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    assert_eq!((1 + 3 + 3) * U64::VALUE_SIZE as u64, key_file_size);
    assert_eq!(0, db_size);
}

#[test]
#[serial]
fn _4d_u64_database_correctly_sizes_files_for_min_link() {
    let db_dimensions = [2, 3, 2, 2];

    cql_db::create_db::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    );

    cql_db::link_dimensions::<U64>(
        DATABASE_LOCATION,
        &[1, 1, 1],
    );

    let key_file_size1_2 = get_file_length(&format!("{}/key1_2", DATABASE_LOCATION));
    let key_file_size2_3 = get_file_length(&format!("{}/key2_3", DATABASE_LOCATION));
    let db_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    assert_eq!((1 + 1) * U64::VALUE_SIZE as u64, key_file_size1_2);
    assert_eq!((1 + 1) * U64::VALUE_SIZE as u64, key_file_size2_3);
    assert_eq!(calculate_database_size(&db_dimensions, 1), db_size);
}

#[test]
#[serial]
fn _4d_u64_database_correctly_sizes_files_for_max_link() {
    let db_dimensions = [2, 3, 2, 2];

    cql_db::create_db::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    );

    cql_db::link_dimensions::<U64>(
        DATABASE_LOCATION,
        &[2, 3, 2],
    );

    let key_file_size1_2 = get_file_length(&format!("{}/key1_2", DATABASE_LOCATION));
    let key_file_size2_3 = get_file_length(&format!("{}/key2_3", DATABASE_LOCATION));
    let db_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    assert_eq!((1 + 3 + 3) * U64::VALUE_SIZE as u64, key_file_size1_2);
    assert_eq!((1 + 2) * U64::VALUE_SIZE as u64, key_file_size2_3);
    assert_eq!(calculate_database_size(&db_dimensions, 1), db_size);
}

#[test]
#[serial]
fn _4d_u64_database_allows_for_mins_to_be_linked() {
    let point1 = [1, 1, 1, 1];
    let value1 = 5;

    cql_db::create_db::<U64>(
        DATABASE_LOCATION,
        &[2, 3, 2, 2]
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

    assert_eq!(value1, result1);
}

#[test]
#[serial]
fn _4d_u64_database_allows_for_maxes_to_be_linked() {
    let point1 = [2, 3, 2, 2];
    let value1 = 5;

    cql_db::create_db::<U64>(
        DATABASE_LOCATION,
        &[2, 3, 2, 2]
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

    assert_eq!(value1, result1);
}

#[test]
#[serial]
fn _4d_u64_database_correctly_sizes_files_for_all_links() {
    let db_dimensions = [2, 3, 2, 2];

    cql_db::create_db::<U64>(
        DATABASE_LOCATION,
        &db_dimensions
    );

    for i in 1..3 {
        for j in 1..4 {
            for k in 1..3 {
                cql_db::link_dimensions::<U64>(
                    DATABASE_LOCATION,
                    &[i, j, k],
                );
            }
        }
    }

    // test extra/duplicate link call
    cql_db::link_dimensions::<U64>(
        DATABASE_LOCATION,
        &[1, 2, 1],
    );

    let key_file_size1_2 = get_file_length(&format!("{}/key1_2", DATABASE_LOCATION));
    let key_file_size2_3 = get_file_length(&format!("{}/key2_3", DATABASE_LOCATION));
    let db_size = get_file_length(&format!("{}/db", DATABASE_LOCATION));

    assert_eq!(calculate_key_file_max_size(&db_dimensions, 0), key_file_size1_2);
    assert_eq!(calculate_key_file_max_size(&db_dimensions, 1), key_file_size2_3);
    assert_eq!(calculate_database_size(&db_dimensions, db_dimensions[0] * db_dimensions[1] * db_dimensions[2]), db_size);
}

#[test]
#[serial]
fn _4d_u64_database_allows_for_all_points_to_be_linked() {
    cql_db::create_db::<U64>(
        DATABASE_LOCATION,
        &[2, 3, 2, 2]
    );

    for i in 1..3 {
        for j in 1..4 {
            for k in 1..3 {
                cql_db::link_dimensions::<U64>(
                    DATABASE_LOCATION,
                    &[i, j, k],
                );
            }
        }
    }

    // test extra/duplicate link call
    cql_db::link_dimensions::<U64>(
        DATABASE_LOCATION,
        &[1, 2, 1],
    );

    for i in 1..3 {
        for j in 1..4 {
            for k in 1..3 {
                for l in 1..3 {
                    cql_db::write_value::<U64>(
                        DATABASE_LOCATION,
                        &[i, j, k, l],
                        i * j * k * l,
                    );
                }
            }
        }
    }

    for i in 1..3 {
        for j in 1..4 {
            for k in 1..3 {
                for l in 1..3 {
                    let result1 = cql_db::read_value::<U64>(
                        DATABASE_LOCATION,
                        &[i, j, k, l]
                    );

                    assert_eq!(result1, i * j * k * l);
                }
            }
        }
    }
}

fn calculate_key_file_max_size(db_dimensions: &[u64], first_dimension_index: usize) -> u64 {
    let mut dimension_size = db_dimensions[0];
    let mut key_file_size = 0;

    for index in 0..first_dimension_index + 1 {
        key_file_size = (1 + (dimension_size * db_dimensions[index + 1])) * U64::VALUE_SIZE as u64;
        dimension_size = dimension_size * db_dimensions[index + 1];
    }

    key_file_size
}

fn calculate_database_size(db_dimensions: &[u64], n_dimensions_linked: u64) -> u64 {
    db_dimensions[db_dimensions.len() - 1] * n_dimensions_linked * U64::VALUE_SIZE as u64
}

fn get_file_length(file_path: &str) -> u64 {
    OpenOptions::new().read(true).open(file_path).unwrap().metadata().unwrap().len()
}
