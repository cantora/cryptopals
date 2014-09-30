
CRYPTOPALS_SRC		= $(shell find cryptopals/ -iname '*.rs')
MDO_CRATE_ROOT		= rust-mdo/src/lib.rs
MDO_SRC				= $(shell find rust-mdo/ -iname '*.rs' 2>/dev/null || echo $(MDO_CRATE_ROOT))

.PHONY: all
all: libcryptopals.rlib libmdo.rlib

libcryptopals.rlib: makefile $(CRYPTOPALS_SRC)
	rm -f libcryptopals*.rlib
	rustc --crate-type rlib cryptopals/lib.rs
	find . -iname 'libcryptopals*.rlib' | head -1 | xargs -I% ln -s % $@


rust-mdo/src/lib.rs:
	git clone 'https://github.com/TeXitoi/rust-mdo.git'
	cd rust-mdo \
		&& git checkout 3fbeb615359af9c705eccf82915e8ea7851f7594

libmdo.rlib: makefile $(MDO_SRC)
	rm -f libmdo*.rlib
	if ! grep 'crate_id = "mdo#0.0.4"' $(MDO_CRATE_ROOT); then \
		{ \
			echo '#![crate_id = "mdo#0.0.4"]'; \
			cat $(MDO_CRATE_ROOT); \
		} > $(MDO_CRATE_ROOT).tmp; \
		cp $(MDO_CRATE_ROOT).tmp $(MDO_CRATE_ROOT); \
	fi

	rustc --crate-type rlib $(MDO_CRATE_ROOT)
	find . -iname 'libmdo*.rlib' | head -1 | xargs -I% ln -s % $@

clean:
	rm -f *.rlib
	rm -rf ./rust-mdo
