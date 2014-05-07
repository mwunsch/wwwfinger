.SUFFIXES:

SRC := src/*.rs

target/wwwfinger: $(SRC)
	mkdir -p $(dir $@)
	rustc -L lib/rust-openssl/build -L lib/rust-http/build --out-dir $(dir $@) $<

clean:
	rm -rf target

.PHONY: clean
