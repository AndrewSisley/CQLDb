mod constants;

use std::io::{ Cursor };
use cql_f64::{ unpack_stream };
pub mod single_point_read_writes;
pub mod stream_read_point_writes;

fn unpack_f64_stream (stream: &mut Cursor<Vec<u8>>, n_values: usize, result: &mut [f64]) {
    unpack_stream(stream, n_values, |idx, value| {
        result[idx] = value
    }).unwrap()
}
