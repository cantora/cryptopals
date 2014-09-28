
SOURCES = $(shell find cryptopals/ -iname '*.rs')

.PHONY: all
all: libcryptopals.rlib

libcryptopals.rlib: makefile $(SOURCES)
	rm -f *.rlib
	rustc --crate-type rlib cryptopals/lib.rs
	find . -iname '*.rlib' | head -1 | xargs -I% ln -s % $@

