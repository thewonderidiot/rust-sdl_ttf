RUSTC ?= rustc

RUST_SRC = $(shell find src/. -type f -name '*.rs')

.PHONY: all
all: libsdl_ttf.dummy

libsdl_ttf.dummy: $(RUST_SRC)
	$(RUSTC) $(RUSTFLAGS) $< -o $@
	touch $@

.PHONY: clean
	rm -f *.so *.dylib *.dll *.dummy *.o
