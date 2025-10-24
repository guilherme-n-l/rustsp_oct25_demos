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
