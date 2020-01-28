#![crate_type = "cdylib"]

use libc::c_void;

#[repr(C)]
#[allow(non_snake_case)]
pub struct JNINativeInterface {
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,
    reserved3: *mut c_void,

    GetVersion: extern fn(env: *mut JNIEnv) -> i32,

    _opaque_data: [u8; 1824]
}

pub type JNIEnv = *const JNINativeInterface;

#[no_mangle]
pub extern fn Java_tests_Test_hello(_jre: *mut JNIEnv, _class: *const c_void) {
    println!("hello from rust");
}
