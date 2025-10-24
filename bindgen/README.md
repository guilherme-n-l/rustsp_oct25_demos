## bindgen/lib.h

```h
#ifndef LIB_H
#define LIB_H

#define C_STRUCT_STR_SIZE 10

// Struct used for communication between C and Rust
struct c_struct {
  char from_c[C_STRUCT_STR_SIZE];
  char from_rust[C_STRUCT_STR_SIZE];
};

// Struct representing a simple Rust-side structure
struct rust_struct {
  int value;
};

// Function: Prints a hello message from C
void hello_world_from_c();

// Function: Creates and returns a c_struct populated with C and Rust data
// Parameters:
//   from_rust - string coming from Rust to be stored in from_rust field
// Returns:
//   A struct c_struct containing "I'm C" in from_c and from_rust value in
//   from_rust
struct c_struct sending_c_struct_to_rust(const char *from_rust);

// Function: Receives a rust_struct from Rust and prints its value
// Parameters:
//   from_rust - pointer to a rust_struct instance
void sending_rust_struct_to_c(const struct rust_struct *from_rust);

#endif // LIB_H

```

## bindgen/lib.c

```c
#include <stdio.h>
#include <string.h>
#include "lib.h"

void hello_world_from_c() { puts("Hello world from C land"); }

struct c_struct sending_c_struct_to_rust(const char *from_rust) {
  struct c_struct my_struct = {"I'm C", "\0"};
  strncpy(my_struct.from_rust, from_rust, C_STRUCT_STR_SIZE);
  return my_struct;
}

void sending_rust_struct_to_c(const struct rust_struct *from_rust) {
  printf("Read your value, it says %d\n", from_rust->value);
}
```

## bindgen/main.rs

```rs
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
```

