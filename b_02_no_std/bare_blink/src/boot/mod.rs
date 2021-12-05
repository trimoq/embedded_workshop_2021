global_asm!(include_str!("boot.s"));

#[no_mangle]
pub unsafe fn _start_rust() -> ! {
    crate::main()
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}