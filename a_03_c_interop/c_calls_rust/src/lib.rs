
#[no_mangle]
pub extern "C" fn add(first: &mut u16, second: &mut u16) -> u16{
    *first + *second
}