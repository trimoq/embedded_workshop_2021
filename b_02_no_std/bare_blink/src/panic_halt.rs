use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    wait_forever();
}

#[no_mangle]
pub fn __aeabi_unwind_cpp_pr0() {}

#[inline(always)]
pub fn wait_forever() -> ! {
    unsafe {
        loop {
            asm!("wfe")
        }
    }
}