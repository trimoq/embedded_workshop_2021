# Installation instructions & Hardware

We focus on the tools needed to develop and run the examples in block C and D.
This 

## Hardware
We use an STM32F411RE-Nucleo board for these examples.
The examples will probably run on most other STM32F4 processors with only minimal modifications.
This board was chosen simply for its good availability at the time we prepared the workshop.

Make sure the jumpers are configured as stated in the manual!

## Linux
Make sure to have a gcc toolchain installed for your host architecture (for the cargo tools).
Optional: Install gdb-multiarch for debugging.


## Windows
We need the Microsoft Build Tools for VS.
You can download them here:
```
https://aka.ms/vs/17/release/vs_BuildTools.exe 
```
Select the C++ Build Tools.

We further need to have the drivers for the ST-Link device set up.
Refer to the official ST-Site or ask during the workshop.

## OSx
Make sure to have a gcc toolchain installed for your host architecture (for the cargo tools).

Optional: Install gdb-multiarch for debugging:
```
$ # GDB
$ brew install armmbed/formulae/arm-none-eabi-gcc
```

## Common
We need some rust specific components and tools:
```
rustup update
rustup component add llvm-tools-preview
rustup target add thumbv7em-none-eabihf
cargo install cargo-binutils cargo-embed cargo-flash cargo-expand
```

# Test
To test your setup, please run the hello_stm32 example as below and make sure you see the user led blink.
```
cd c_01_hello_stm32/hello_stm32 
cargo flash --chip stm32f411re --connect-under-reset --release
```