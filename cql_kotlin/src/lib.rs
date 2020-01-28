#![crate_type = "cdylib"]

use libc::c_void;

#[no_mangle]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Java_tests_StorageType {
    F64Nullable = 0,
    TinyText = 1,
}

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

#[repr(C)]
#[allow(non_snake_case)]
pub struct Java_tests_AxisDefinition {
    pub id: u64,
	pub max: u64,
}

#[no_mangle]
pub extern fn Java_tests_Test_hello(_jre: *mut JNIEnv, _class: *const c_void, v: Java_tests_StorageType, ax: *mut Java_tests_AxisDefinition) -> Java_tests_StorageType {
    unsafe {
        let axis = &(*ax);
        println!("hello from rust {:?} {} {}", v, axis.id, axis.max);
    }
    v
}
