use std::{
    ffi::{CStr, c_char},
    fmt::{self, Formatter},
};

/// The size of the strings passed between Rust and C. (Includes NULL terminator)
const C_STRUCT_STRING_SIZE: usize = 10;

#[link(name = "lib")]
unsafe extern "C" {
    /// Calls a C function to print from C.
    fn hello_world_from_c();

    /// Receives a string from Rust, and returns a `CStruct` containing data from C and Rust.
    ///
    /// # Arguments
    /// * `from_rust` - A pointer to a null-terminated string passed from Rust.
    fn sending_c_struct_to_rust(from_rust: *const c_char) -> CStruct;

    /// Sends a Rust struct to C.
    ///
    /// # Arguments
    /// * `from_rust` - A reference to the `RustStruct` to be passed to C.
    fn sending_rust_struct_to_c(from_rust: &RustStruct);
}

/// A struct representing a data structure that can be passed between Rust and C.
///
/// The struct holds two fixed-size C strings, one representing data from C (`from_c`),
/// and another representing data from Rust (`from_rust`).
#[repr(C)]
struct CStruct {
    from_c: [c_char; C_STRUCT_STRING_SIZE],
    from_rust: [c_char; C_STRUCT_STRING_SIZE],
}

#[repr(C)]
struct RustStruct {
    value: i32,
}

impl CStruct {
    /// Converts a C string (`c_char` array) to a Rust `&str`.
    ///
    /// # Arguments
    /// * `field` - A slice of `c_char` representing a C string.
    ///
    /// # Returns
    /// A string slice (`&str`) representing the converted C string.
    fn __convert_str(field: &[c_char]) -> &str {
        unsafe { CStr::from_ptr(field.as_ptr()) }.to_str().unwrap()
    }
}

impl fmt::Debug for CStruct {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "CStruct {{ from_c: \"{}\", from_rust: \"{}\" }}",
            Self::__convert_str(&self.from_c),
            Self::__convert_str(&self.from_rust)
        )
    }
}

/// Safely calls the `hello_world_from_c` function, which prints a message from C.
fn safe_hello_world_from_c() {
    unsafe { hello_world_from_c() }
}

/// Safely sends a `RustStruct` to C.
///
/// # Arguments
/// * `from_rust` - A reference to the `RustStruct` to be sent to C.
fn safe_sending_rust_struct_to_c(from_rust: &RustStruct) {
    unsafe { sending_rust_struct_to_c(from_rust) }
}


/// Safely calls the C function `sending_c_struct_to_rust` to send a C string from Rust to C,
/// and returns a `CStruct` containing the result.
///
/// # Arguments
/// * `from_rust` - A C string passed from Rust to C.
/// 
/// # Returns
/// * `Ok(CStruct)` if the C string fits within the allocated size.
/// * `Err(())` if the string exceeds the allocated size.
fn safe_sending_c_struct_to_rust(from_rust: &CStr) -> Result<CStruct, ()> {
    if from_rust.count_bytes() > C_STRUCT_STRING_SIZE - 1 {
        return Err(());
    }
    Ok(unsafe { sending_c_struct_to_rust(from_rust.as_ptr()) })
}

fn main() {
    safe_hello_world_from_c();
    let c_struct = safe_sending_c_struct_to_rust(c"From rust").unwrap();
    dbg!(c_struct);
    safe_sending_rust_struct_to_c(&RustStruct { value: 32 });
}
