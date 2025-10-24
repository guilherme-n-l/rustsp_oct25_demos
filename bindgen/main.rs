mod bindings {
    include!("bindings_generated.rs");
}

use std::
    ffi::{CStr, c_char}
;

fn __convert_str(field: &[c_char]) -> &str {
    unsafe { CStr::from_ptr(field.as_ptr()) }.to_str().unwrap()
}

fn safe_hello_world_from_c() {
    unsafe { bindings::hello_world_from_c() }
}

fn safe_sending_rust_struct_to_c(from_rust: &bindings::rust_struct) {
    unsafe { bindings::sending_rust_struct_to_c(from_rust) }
}

fn safe_sending_c_struct_to_rust(from_rust: &CStr) -> Result<bindings::c_struct, ()> {
    if from_rust.count_bytes() > ((bindings::C_STRUCT_STR_SIZE - 1) as usize) {
        return Err(());
    }
    Ok(unsafe { bindings::sending_c_struct_to_rust(from_rust.as_ptr()) })
}

fn main() {
    safe_hello_world_from_c();
    let c_struct = safe_sending_c_struct_to_rust(c"From rust").unwrap();
    dbg!(c_struct);
    safe_sending_rust_struct_to_c(&bindings::rust_struct { value: 32 });
}
