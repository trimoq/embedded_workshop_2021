#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f4xx_hal::{
    pac,
    prelude::*,
    delay::Delay
};

#[entry]
fn main() -> ! {
    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();
    let _ = device.SYSCFG.constrain();

    let gpioa = device.GPIOA.split();

    let mut led = gpioa.pa5.into_push_pull_output();
    let mut delay = Delay::new(core.SYST, &clocks);
    
    let mut ctr = 0u16;
    loop {
        ctr+=1;
        led.toggle();
        delay.delay_ms(ctr%200);
    }
}
