#![no_main]
#![no_std]


use core::{cell::RefCell, sync::atomic::{AtomicU16, Ordering}};

use cortex_m::{interrupt::Mutex};
use cortex_m_rt::entry;
use panic_halt as _;

use stm32f4xx_hal::{delay::Delay, gpio::{Edge, gpioc::PC13, Input, PullUp}, interrupt, pac, prelude::*};
use rtt_target::{rtt_init_print, rprintln};


// TODO definitions here


#[entry]
fn main() -> ! {
    rtt_init_print!();

    // TODO setup the board and create the button

    let mut delay = Delay::new(core.SYST, &clocks);

    // TODO unmask the interrupt

    loop {
        delay.delay_ms(200u8);
        // TODO use the `rprintln` macro to print the atomics value from the main loop 
    }
}

#[interrupt]
fn EXTI15_10() {

    // TODO clear the interrupt pending bit 
    
    // TODO increment and print the atomic from within the isr  
}
