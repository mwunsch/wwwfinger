.SUFFIXES:

SRC := src/*.rs
OBJDIR := target
OBJECT := $(OBJDIR)/wwwfinger

RUST_OPENSSL = lib/rust-openssl/build
RUST_HTTP = lib/rust-http/build

$(OBJECT): $(SRC) $(RUST_OPENSSL) $(RUST_HTTP) | $(OBJDIR)
	rustc -L $(RUST_OPENSSL) -L $(RUST_HTTP) --out-dir $| $<

$(OBJDIR):
	mkdir -p $(OBJDIR)

$(RUST_OPENSSL):
	cd $(dir $@) && make

$(RUST_HTTP): $(RUST_OPENSSL) lib/rust-http/Makefile
	cd $(dir $@) && make

lib/rust-http/Makefile:
	cd $(dir $@) && ./configure

.PHONY: all deps sync clean cleandeps cleanall

all: $(OBJECT)

deps: $(RUST_HTTP)

sync:
	git submodule sync

clean:
	rm -rf $(OBJDIR)

cleandeps:
	cd lib/rust-openssl && make clean
	cd lib/rust-http && make clean

cleanall: clean cleandeps

