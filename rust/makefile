
CRYPTOPALS_SRC		= $(shell find cryptopals/ -iname '*.rs')

STATIC_MDO_LIBFILE	= static_mdo/src/lib.rs
STATIC_MDO_SRC		= $(shell find static_mdo/ -iname '*.rs' 2>/dev/null || echo $(STATIC_MDO_LIBFILE) )

RUSTOPENSSL_DIR     = rust-openssl

.PHONY: all
all: .libcryptopals.rlib libstatic_mdo.rlib libopenssl.rlib

.libcryptopals.rlib: makefile $(CRYPTOPALS_SRC) libopenssl.rlib
	rm -f libcryptopals*.rlib
	rustc -L . --crate-type rlib cryptopals/lib.rs
	touch $@

.PHONY: expanded-cryptopals
expanded-cryptopals:
	rustc --pretty expanded --crate-type rlib cryptopals/lib.rs

$(STATIC_MDO_LIBFILE):
	git clone 'https://github.com/cantora/static_mdo.git'

libstatic_mdo.rlib: $(STATIC_MDO_SRC)
	cd static_mdo && cargo build
	ln -sf static_mdo/libstatic_mdo.rlib

$(RUSTOPENSSL_DIR)/src/lib.rs:
	tmp=$$(mktemp) \
	  && git clone 'https://github.com/sfackler/rust-openssl.git' $$tmp/rust-openssl \
	  && cd $$tmp/rust-openssl \
	  && git checkout b084ddc7b0fe9c7816fef6e415a05db6ec43fb71 \
	  && mv $$tmp/rust-openssl $(RUSTOPENSSL_DIR)

$(RUSTOPENSSL_DIR)/libopenssl.rlib: $(RUSTOPENSSL_DIR)/src/lib.rs
	cd $(RUSTOPENSSL_DIR) && cargo build

libopenssl.rlib: $(RUSTOPENSSL_DIR)/libopenssl.rlib
	ln -s $<

clean:
	rm -f *.rlib
	rm -f .*.rlib
