#![feature(lang_items)]
#![feature(start)]
#![feature(asm)]
#![no_std]



pub fn write(fd: u32, buf: *const u8, count: usize) {
    let syscall_number: u64 = 1;
    unsafe{
        asm!(
            "syscall",
            in("rax") syscall_number,
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") count,
            // Linux syscalls don't touch the stack at all, so
            // we don't care about its alignment
            options(nostack)
        );
    }    
}

pub fn exit(code: i32) -> ! {
    let syscall_number: u64 = 60;
    unsafe{
        asm!(
            "syscall",
            in("rax") syscall_number,
            in("rdi") code,
            options(noreturn)
        );
    }
}

#[no_mangle]
#[start]
pub fn _start(_nargs: isize, _args: *const *const u8) -> isize {
    write(1, b"Hello, World!\n" as *const u8, 14);
    exit(0);    
}

#[lang = "eh_personality"] 
extern fn eh_personality() {}


#[panic_handler] 
fn panic(_info: &core::panic::PanicInfo) -> ! { loop{} }