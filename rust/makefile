
CRYPTOPALS_SRC		= $(shell find cryptopals/ -iname '*.rs')

STATIC_MDO_LIBFILE	= static_mdo/src/lib.rs
STATIC_MDO_SRC		= $(shell find static_mdo/ -iname '*.rs' 2>/dev/null || echo $(STATIC_MDO_LIBFILE) )

.PHONY: all
all: .libcryptopals.rlib .libstatic_mdo.rlib

.libcryptopals.rlib: makefile $(CRYPTOPALS_SRC)
	rm -f libcryptopals*.rlib
	rustc --crate-type rlib cryptopals/lib.rs
	touch $@

.PHONY: expanded-cryptopals
expanded-cryptopals:
	rustc --pretty expanded --crate-type rlib cryptopals/lib.rs

$(STATIC_MDO_LIBFILE):
	git clone 'https://github.com/cantora/static_mdo.git'

.libstatic_mdo.rlib: makefile $(STATIC_MDO_SRC)
	rm -f libstatic_mdo*.rlib
	$(MAKE) -C static_mdo
	ln -s static_mdo/$$(readlink static_mdo/libstatic_mdo.rlib)
	touch $@

clean:
	rm -f *.rlib
	rm -f .*.rlib
