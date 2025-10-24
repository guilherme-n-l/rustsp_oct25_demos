BINDGEN = bindgen
GENERATED = bindings_generated.rs
CC = clang
LDFLAGS = -L.
RUSTC = rustc
RUSTFLAGS = +stable --edition 2021 -L . -l$(patsubst lib%,%,$(LIBNAME))
LIBSRC = lib.c
LIBH = lib.h
LIBNAME = liblib
LIBFLAGS = -I.
SRC = main.rs
MAIN = main
UNAME_S := $(shell uname -s)

ifeq ($(UNAME_S),Darwin)
    LIB_EXT = dylib
    LIBFLAGS += -dynamiclib
else ifeq ($(UNAME_S),Linux)
    LIB_EXT = so
    LIBFLAGS += -fPIC -shared
else
    $(error Unsupported OS: $(UNAME_S))
endif

LIB = $(LIBNAME).$(LIB_EXT)

all: bindings $(LIB) $(MAIN)

bindings: $(GENERATED)
$(GENERATED): $(LIBH)
	$(BINDGEN) $^ > $@

$(LIBNAME): $(LIB)
$(LIB): $(LIBSRC)
	$(CC) $(LIBFLAGS) $(LDFLAGS) -o $@ $^

$(MAIN): $(SRC)
	$(RUSTC) $(RUSTFLAGS) -o $@ $^

clean:
	rm -f $(LIB) $(MAIN) $(GENERATED) target

.PHONY: all bindings clean $(LIBNAME)
