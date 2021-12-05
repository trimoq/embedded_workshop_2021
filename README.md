# Tools and setup
See [here](INSTALL.md).

# Block A
First Block: Running on and integrating with desktop OSes.

## `File`
- [Makefile](a_01_file/Makefile)
- Comparison of failure scenarios of C and Rust
- Platform agnositc APIs in C and Rust
- Highlight of problems with glibc versions

## `syscall`
- Quick [example](a_02_syscall/src/bin/pointer.rs) showing basic usage of pointers
- Quick-and-dirty [example](a_02_syscall/src/bin/sloppy.rs) to show that rust is no magic sauce (but the compiler helps)
- The same [example](a_02_syscall/src/bin/better.rs) again but with a bit more careful error handling in place.

## C-Interoperability

### Rust calls C
- [Showcase](a_03_c_interop/rust_calls_c/Makefile) of how a rust application can use an external C dependency.
  This was already done under the hood by the syscall example.
- We build the example with gcc

### C calls Rust
- Basic [example](a_03_c_interop/c_calls_rust/Makefile) of a C application making use of some Rust library
- We use cbindgen to generate bindings to the Rust lib to expose a C API.

### Bindgen + cc
- [Showcase](a_03_c_interop/generated_sys/build.rs) of combining `bindgen` and `cc` to compile a C library and create bindigns to it for Rust.
- These are two crates `generated` and `generated_sys` to mirror the separation found in most projects 

## Pi
We move to a different plattform, the RaspberryPi

### blink
- The typical Hello World [example](a_04_pi/blink/Cargo.toml) used to make an LED blink
- We make use of a reaedy-made library `rppal` but encounter challenges with crosscompiling

### blink_mem
- How does the `rppal` library make the led blink?
- We poke around the library to find out that we need to set some registers
- We use `/dev/mem` to write our own [code](a_04_pi/pi_mem/Makefile) doing so
- This is in preparation of a later example

# Block B

## Rust Java interop
- Interoperability of Java and Rust
- Small [example](b_01_java_cpp_interop/java_interop/Makefile) of an integration between Java and Rust.
  This will be reused in the Android example

## Rust C++ interop
- Interoperability of C++ and Rust
- [Showcase](b_01_java_cpp_interop/cpp_interop/build.rs) demonstrating the use of the cxx crate.
- Whilst this is more powerful than the plain C interoperability, we also see some of the limitations. 

## `no_std`

### No_std hello world
- After we learned where parts of the Rust ecosystem are located, we can make use of those present in the no_std world.
- Printing `Hello World` with `no_std` but using the OS nonetheless ist ridiculous and surprisingly hard
- We learn to get a feeling for no_std and its limitations but see some of Rusts beauty: The mighty features like the Ownership system can also be used in a `no_std` environment.
- Since we ditched the standard library, our resulting executable is tiny

### bare_blink
- Remember the code from the `blink`[blink_mem](a_04_pi/pi_mem/Makefile) making the LED blink by poking around the memory? We can do this without the OS with nearly the same [code](b_02_no_std/bare_blink/Makefile)! Of course we need to provide a bit of glue around it, after all, we write a exceptionally simple OS-Kernel.

# Block C & D