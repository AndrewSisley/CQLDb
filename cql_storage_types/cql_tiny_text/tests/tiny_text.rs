mod constants;

use std::io::{ Cursor };
use cql_tiny_text::{ TinyText, unpack_stream };
pub mod single_point_read_writes;
pub mod stream_read_point_writes;

fn unpack_tiny_text_stream (stream: &mut Cursor<Vec<u8>>, n_values: usize, result: &mut [TinyText]) {
    unpack_stream(stream, n_values, |idx, value| {
        result[idx] = value
    }).unwrap()
}
