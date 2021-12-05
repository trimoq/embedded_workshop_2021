fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/points.cpp")
        .compile("cpp_interop");
}