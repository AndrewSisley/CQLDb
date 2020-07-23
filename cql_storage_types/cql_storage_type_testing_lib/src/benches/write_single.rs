use cql_model::{ CqlWritable };

pub fn _1d_write_location_1<'a, TStore: CqlWritable>(db_location: &'a str) -> Box<dyn Fn(TStore::ValueType) + 'a> {
    let axis = [
        1,
    ];

    static POINT1: [u64; 1] = [1];

    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &axis
    ).unwrap();

    Box::new(move |value| -> () {
        cql_db::write_value_unchecked::<TStore>(
            db_location,
            &POINT1,
            value
        ).unwrap();
    })
}

pub fn _1d_write_location_100000<'a, TStore: CqlWritable>(db_location: &'a str) -> Box<dyn Fn(TStore::ValueType) + 'a> {
    let axis = [
        100000,
    ];

    static POINT1: [u64; 1] = [100000];

    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &axis
    ).unwrap();

    Box::new(move |value| -> () {
        cql_db::write_value_unchecked::<TStore>(
            db_location,
            &POINT1,
            value
        ).unwrap();
    })
}
