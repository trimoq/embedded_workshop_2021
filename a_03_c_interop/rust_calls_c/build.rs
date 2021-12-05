fn main() {
    println!("cargo:rustc-link-search=./lib/basic/");
    println!("cargo:rustc-link-search=./lib/point/");
}