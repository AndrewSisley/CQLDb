#![feature(test)]
mod constants;
extern crate test;

use std::io::{ Read, Cursor, SeekFrom, Seek };
use byteorder::{ ReadBytesExt, LittleEndian };
use constants::DATABASE_LOCATION;
use test::{ Bencher };

#[bench]
fn _1d_f64_nullable_stream_read_empty_location_1_to_1(b: &mut Bencher) {
    let axis = [
        cql_db::AxisDefinition {
            id: 1,
            max: 2,
        },
    ];

    let n_values_to_read = 1usize;
    let point1 = [1];

    cql_db::create_db::<Option<f64>>(
        DATABASE_LOCATION,
        &axis
    );

    let mut result: [Option<f64>; 1] = [None];
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream::<Option<f64>>(
            DATABASE_LOCATION,
            &mut stream,
            &point1,
            n_values_to_read as u64
        );

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, n_values_to_read, |idx, value| {
            result[idx] = value
        });
    });
}

#[bench]
fn _1d_f64_nullable_stream_read_populated_location_1_to_1(b: &mut Bencher) {
    let axis = [
        cql_db::AxisDefinition {
            id: 1,
            max: 2,
        },
    ];

    let n_values_to_read = 1usize;
    let point1 = [1];
    let value1 = Some(42.87);

    cql_db::create_db::<Option<f64>>(
        DATABASE_LOCATION,
        &axis
    );

    cql_db::write_value::<Option<f64>>(
        DATABASE_LOCATION,
        &point1,
        value1
    );

    let mut result: [Option<f64>; 1] = [None];
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream::<Option<f64>>(
            DATABASE_LOCATION,
            &mut stream,
            &point1,
            n_values_to_read as u64
        );

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, n_values_to_read, |idx, value| {
            result[idx] = value
        });
    });
}

#[bench]
fn _1d_f64_nullable_stream_read_empty_location_50000_to_100000(b: &mut Bencher) {
    let axis = [
        cql_db::AxisDefinition {
            id: 1,
            max: 100000,
        },
    ];

    let n_values_to_read = 50000usize;
    let point1 = [50000];

    cql_db::create_db::<Option<f64>>(
        DATABASE_LOCATION,
        &axis
    );

    let mut result: [Option<f64>; 50000] = [None; 50000];
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream::<Option<f64>>(
            DATABASE_LOCATION,
            &mut stream,
            &point1,
            n_values_to_read as u64
        );

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, n_values_to_read, |idx, value| {
            result[idx] = value
        });
    });
}

#[bench]
fn _1d_f64_nullable_stream_read_populated_location_50000_to_100000(b: &mut Bencher) {
    let axis = [
        cql_db::AxisDefinition {
            id: 1,
            max: 100000,
        },
    ];

    let n_values_to_read = 50000usize;
    let base_point = [50000u64];
    let base_value = 42.87f64;

    cql_db::create_db::<Option<f64>>(
        DATABASE_LOCATION,
        &axis
    );

    for index in 0..n_values_to_read {
        cql_db::write_value::<Option<f64>>(
            DATABASE_LOCATION,
            &[base_point[0] + index as u64],
            Some(base_value + index as f64)
        );
    }

    let mut result: [Option<f64>; 50000] = [None; 50000];
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream::<Option<f64>>(
            DATABASE_LOCATION,
            &mut stream,
            &base_point,
            n_values_to_read as u64
        );

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, n_values_to_read, |idx, value| {
            result[idx] = value
        });
    });
}

#[bench]
fn _4d_f64_nullable_stream_read_empty_location_1_1_1_1_to_1_1_1_1(b: &mut Bencher) {
    let axis = [
        cql_db::AxisDefinition {
            id: 1,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 2,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 3,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 4,
            max: 2
        },
    ];
    let n_values_to_read = 1usize;
    let base_point = [1, 1, 1, 1];

    cql_db::create_db::<Option<f64>>(
        DATABASE_LOCATION,
        &axis
    );

    let first_to_second_key = cql_db::add_key::<Option<f64>>(
        DATABASE_LOCATION,
        base_point[0],
        base_point[1],
        &axis[0],
        &axis[1]
    );

    cql_db::add_key::<Option<f64>>(
        DATABASE_LOCATION,
        first_to_second_key,
        base_point[2],
        &axis[1],
        &axis[2]
    );

    let mut result: [Option<f64>; 1] = [None; 1];
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream::<Option<f64>>(
            DATABASE_LOCATION,
            &mut stream,
            &base_point,
            n_values_to_read as u64
        );

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, n_values_to_read, |idx, value| {
            result[idx] = value
        });
    });
}

#[bench]
fn _4d_f64_nullable_stream_read_populated_location_1_1_1_1_to_1_1_1_1(b: &mut Bencher) {
    let axis = [
        cql_db::AxisDefinition {
            id: 1,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 2,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 3,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 4,
            max: 2
        },
    ];

    let n_values_to_read = 1usize;
    let base_point = [1, 1, 1, 1];
    let base_value = Some(78352.3);

    cql_db::create_db::<Option<f64>>(
        DATABASE_LOCATION,
        &axis
    );

    let first_to_second_key = cql_db::add_key::<Option<f64>>(
        DATABASE_LOCATION,
        base_point[0],
        base_point[1],
        &axis[0],
        &axis[1]
    );

    cql_db::add_key::<Option<f64>>(
        DATABASE_LOCATION,
        first_to_second_key,
        base_point[2],
        &axis[1],
        &axis[2]
    );

    cql_db::write_value::<Option<f64>>(
        DATABASE_LOCATION,
        &base_point,
        base_value
    );

    let mut result: [Option<f64>; 1] = [None; 1];
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream::<Option<f64>>(
            DATABASE_LOCATION,
            &mut stream,
            &base_point,
            n_values_to_read as u64
        );

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, n_values_to_read, |idx, value| {
            result[idx] = value
        });
    });
}

#[bench]
fn _4d_f64_nullable_stream_read_empty_location_1_1_1_50000_to_1_1_1_100000(b: &mut Bencher) {
    let axis = [
        cql_db::AxisDefinition {
            id: 1,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 2,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 3,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 4,
            max: 100000
        },
    ];

    let n_values_to_read = 50000usize;
    let base_point = [1, 1, 1, 1];

    cql_db::create_db::<Option<f64>>(
        DATABASE_LOCATION,
        &axis
    );

    let first_to_second_key = cql_db::add_key::<Option<f64>>(
        DATABASE_LOCATION,
        base_point[0],
        base_point[1],
        &axis[0],
        &axis[1]
    );

    cql_db::add_key::<Option<f64>>(
        DATABASE_LOCATION,
        first_to_second_key,
        base_point[2],
        &axis[1],
        &axis[2]
    );

    let mut result: [Option<f64>; 50000] = [None; 50000];
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream::<Option<f64>>(
            DATABASE_LOCATION,
            &mut stream,
            &base_point,
            n_values_to_read as u64
        );

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, n_values_to_read, |idx, value| {
            result[idx] = value
        });
    });
}

#[bench]
fn _4d_f64_nullable_stream_read_populated_location_1_1_1_50000_to_1_1_1_100000(b: &mut Bencher) {
    let axis = [
        cql_db::AxisDefinition {
            id: 1,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 2,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 3,
            max: 2,
        },
        cql_db::AxisDefinition {
            id: 4,
            max: 100000
        },
    ];

    let n_values_to_read = 50000usize;
    let base_point = [1, 1, 1, 1];
    let base_value = 78352.3;

    cql_db::create_db::<Option<f64>>(
        DATABASE_LOCATION,
        &axis
    );

    let first_to_second_key = cql_db::add_key::<Option<f64>>(
        DATABASE_LOCATION,
        base_point[0],
        base_point[1],
        &axis[0],
        &axis[1]
    );

    cql_db::add_key::<Option<f64>>(
        DATABASE_LOCATION,
        first_to_second_key,
        base_point[2],
        &axis[1],
        &axis[2]
    );

    for index in 0..n_values_to_read {
        cql_db::write_value::<Option<f64>>(
            DATABASE_LOCATION,
            &[1, 1, 1, base_point[0] + index as u64],
            Some(base_value + index as f64)
        );
    }

    let mut result: [Option<f64>; 50000] = [None; 50000];
    let mut stream = Cursor::new(Vec::new());

    b.iter(|| {
        cql_db::read_to_stream::<Option<f64>>(
            DATABASE_LOCATION,
            &mut stream,
            &base_point,
            n_values_to_read as u64
        );

        stream.seek(SeekFrom::Start(0)).unwrap();

        unpack_stream(&mut stream, n_values_to_read, |idx, value| {
            result[idx] = value
        });
    });
}

fn unpack_stream<F>(stream: &mut Cursor<Vec<u8>>, n_values: usize, mut res: F) where F: FnMut(usize, Option<f64>) {
    for index in 0..n_values {
        let mut null_buffer = [0; 1];
        let mut value_buffer = [0; 8];

        match stream.read(&mut null_buffer) {
            Ok(n) => {
                if n == 0 { break; }
                else if null_buffer[0] == 2 { break; }
                else if null_buffer[0] == 0 {
                    stream.read(&mut value_buffer).unwrap();
                    res(index, None);
                }
                else {
                    let mut value_buffer = [0; 8];
                    stream.read(&mut value_buffer).unwrap();

                    let mut rdr = Cursor::new(value_buffer);
                    res(index, Some(rdr.read_f64::<LittleEndian>().unwrap()));
                }
            },
            Err(_) => panic!()
        }
    }
}