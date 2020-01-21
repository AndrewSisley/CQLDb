use std::fs::OpenOptions;

pub fn grow_database(db_location: &str, size_to_grow: u64, value_size: u64) {
    // FIXME: Need to make sure that this operation is atomic.
    let file = OpenOptions::new().write(true).open(db_location).unwrap();
    file.set_len(file.metadata().unwrap().len() + size_to_grow * value_size).unwrap();
}
