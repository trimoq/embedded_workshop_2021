# Installation instructions & Hardware

We focus on the tools needed to develop and run the examples in block C and D.
This 

## Hardware
We use an STM32F411RE-Nucleo board for these examples.
The examples will probably run on most other STM32F4 processors with only minimal modifications.
This board was chosen simply for its good availability at the time we prepared the workshop.

## Linux


## Windows

## OSx

## Common
```
rustup component add llvm-tools-preview
rustup toolchain install thumbv7em-none-eabihf
rustup target add thumbv7em-none-eabihf
cargo install cargo-binutils cargo-embed cargo-flash cargo-expand rust-objcopy cbindgen
```