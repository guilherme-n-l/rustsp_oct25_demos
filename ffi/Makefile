CC = clang
RUSTC = rustc
RUSTFLAGS =  +stable --edition 2021 -L .
LIBSRC = lib.c
LIBNAME = liblib
SRC = main.rs
MAIN = main
UNAME_S := $(shell uname -s)

ifeq ($(UNAME_S),Darwin)
    LIB_EXT = dylib
    LIBFLAGS = -dynamiclib
else ifeq ($(UNAME_S),Linux)
    LIB_EXT = so
    LIBFLAGS = -fPIC -shared
else
    $(error Unsupported OS: $(UNAME_S))
endif


LIB = $(LIBNAME).$(LIB_EXT)

all: $(LIBNAME) $(MAIN)

$(LIBNAME): $(LIB)

$(LIB): $(LIBSRC)
	$(CC) $(LIBFLAGS) -o $@ $^

$(MAIN): $(SRC)
	$(RUSTC) $(RUSTFLAGS) -o $@ $^

clean:
	rm -f $(LIB) $(MAIN) target

.PHONY: all clean $(LIBNAME)
