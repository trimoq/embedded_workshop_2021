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
    let tx_pin = gpioa.pa2.into_alternate();
    let serial_config = Config::default().baudrate(9600.bps());
    let usart = device.USART2;

    // configure serial
    let mut tx = Serial::tx(
        usart,
        tx_pin,
        serial_config,
        clocks,
    ).unwrap();


    let mut ctr = 0u8;    
    loop {
        writeln!(tx, "value: {:02}\r", ctr).unwrap();
        ctr+=1;
        delay.delay_ms(50u16);
    }
}
