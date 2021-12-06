#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f4xx_hal::{
    pac,
    prelude::*,
    delay::Delay
};

use rtt_target::{rtt_init_print, rprintln};


#[entry]
fn main() -> ! {
    rtt_init_print!();

    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();
    let _ = device.SYSCFG.constrain();

    let gpioa = device.GPIOA.split();

    let mut led = gpioa.pa5.into_push_pull_output();
    let mut delay = Delay::new(core.SYST, &clocks);
    
    rprintln!("Hello, world!"); 

    loop {
        
        // TODO print and increment a counter, use the `rprintln` macro

        led.toggle();
        delay.delay_ms(200u16);
    }
}
