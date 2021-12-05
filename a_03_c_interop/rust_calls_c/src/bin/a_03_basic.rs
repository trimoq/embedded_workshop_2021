use std::os::raw::c_int;


#[link(name = "basic")]
extern "C" {
    fn basic_function(i: c_int) -> c_int;
}


fn main() {
    let value = 42;
    let result = unsafe{ basic_function(value) };
    println!("Result: {}", result);
}
