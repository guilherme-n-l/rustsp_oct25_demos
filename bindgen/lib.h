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

