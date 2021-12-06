#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f4xx_hal::{
    pac,
    prelude::*,
    serial::config::Config, 
    serial::Serial,
    delay::Delay
};

use core::fmt::Write; // for pretty formatting of the serial output


#[entry]
fn main() -> ! {
    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();
    let _ = device.SYSCFG.constrain();
    let mut delay = Delay::new(core.SYST, &clocks);

    let gpioa = device.GPIOA.split();
    
    /* TODO
    * Setup PA2 as tx pin
    * use USART2 as usart device
    * use the default config with a baudrate of 9600 bps
    * Create a serial device with `Serial::tx`
    */


    loop {
        // TODO write counter value with the `writeln!` macro to out serial device
        delay.delay_ms(50u16);
    }
}
