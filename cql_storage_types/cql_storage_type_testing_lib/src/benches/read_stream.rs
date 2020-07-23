use std::io::{ Cursor, SeekFrom, Seek };
use cql_model::{ CqlStreamReadable };

pub fn _1d_read_empty_location_1_to_1<'a, TStore: CqlStreamReadable>(
            db_location: &'a str,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        ) -> impl Fn() + 'a
        where TStore::ValueType: Default {
    const N_VALUES_TO_READ: usize = 1;
    static POINT1: [u64; 1] = [1];

    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &[1]
    ).unwrap();

    move || -> () {
        let mut result = Vec::with_capacity(N_VALUES_TO_READ);
        result.resize_with(N_VALUES_TO_READ, Default::default);
        let mut stream = Cursor::new(Vec::new());

        cql_db::read_to_stream_unchecked::<TStore>(
            db_location,
            &mut stream,
            &POINT1,
            N_VALUES_TO_READ as u64
        ).unwrap();

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, N_VALUES_TO_READ, &mut result);
    }
}

pub fn _1d_read_empty_location_50000_to_100000<'a, TStore: CqlStreamReadable>(
            db_location: &'a str,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        ) -> impl Fn() + 'a
        where TStore::ValueType: Default {
    const N_VALUES_TO_READ: usize = 50000;
    static BASE_POINT: [u64; 1] = [50000];

    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &[100000]
    ).unwrap();

    move || -> () {
        let mut result = Vec::with_capacity(N_VALUES_TO_READ);
        result.resize_with(N_VALUES_TO_READ, Default::default);
        let mut stream = Cursor::new(Vec::new());

        cql_db::read_to_stream_unchecked::<TStore>(
            db_location,
            &mut stream,
            &BASE_POINT,
            N_VALUES_TO_READ as u64
        ).unwrap();

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, N_VALUES_TO_READ, &mut result);
    }
}

pub fn _4d_read_empty_location_1_1_1_1_to_1_1_1_1<'a, TStore: CqlStreamReadable>(
            db_location: &'a str,
            unpack_stream: &'a dyn Fn(&mut Cursor<Vec<u8>>, usize, &mut [TStore::ValueType])
        ) -> impl Fn() + 'a
        where TStore::ValueType: Default {
    const N_VALUES_TO_READ: usize = 1;
    static BASE_POINT: [u64; 4] = [1, 1, 1, 1];

    cql_db::create_db_unchecked::<TStore>(
        db_location,
        &[1, 1, 1, 1]
    ).unwrap();

    cql_db::link_dimensions_unchecked::<TStore>(
        db_location,
        &BASE_POINT[0..3],
    ).unwrap();

    move || -> () {
        let mut result = Vec::with_capacity(N_VALUES_TO_READ);
        result.resize_with(N_VALUES_TO_READ, Default::default);
        let mut stream = Cursor::new(Vec::new());

        cql_db::read_to_stream_unchecked::<TStore>(
            db_location,
            &mut stream,
            &BASE_POINT,
            N_VALUES_TO_READ as u64
        ).unwrap();

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, N_VALUES_TO_READ, &mut result);
    }
}
