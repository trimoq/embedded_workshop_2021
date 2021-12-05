all: build_ext

dirs:
	mkdir -p ext/lib
	mkdir -p ext/include

bindgen: src/lib.rs dirs cbindgen.toml
	cbindgen --config cbindgen.toml --crate c_calls_rust --output ext/include/my_header.h

build_rust: bindgen dirs
	cargo build --release
	cp target/release/libc_calls_rust.a ext/lib

build_ext: build_rust
	make -C ext

clean:
	make -C ext clean
	cargo clean