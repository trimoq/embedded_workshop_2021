#![feature(lang_items)]
#![feature(asm)]
#![feature(global_asm)]
#![no_main]
#![no_std]

use led::Led23;

mod boot;
mod led;
mod panic_halt;


#[inline(always)]
pub fn spin_for_cycles(n: usize) {
    unsafe {
        for _ in 0..n {
            asm!("nop")
        }
    }
}

fn main() -> ! {

    let mut led = Led23::new();

    loop {
        led.toggle();
        spin_for_cycles(500_000);
    }
}

