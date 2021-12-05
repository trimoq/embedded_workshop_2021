.PHONY: all elf bin

all: bin

elf:
	RUSTFLAGS="-C link-arg=-Tsrc/boot/link.ld -C target-cpu=cortex-a7" cargo +nightly rustc --target=armv7-unknown-linux-gnueabihf --release 


bin: elf
	/home/marco/.cargo/bin/rust-objcopy --strip-all -O binary target/armv7-unknown-linux-gnueabihf/release/kernel kernel7.img