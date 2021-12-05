#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use core::fmt::Write;

use stm32f4xx_hal::{
    pac, 
    delay::Delay, 
    prelude::*,
    serial::Serial, 
    serial::config::Config,
    adc::{
        Adc,
        config::AdcConfig,
        config::SampleTime,
      },
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

    let tx_pin = gpioa.pa2.into_alternate();
    let mut tx = Serial::tx(
        device.USART2,
        tx_pin,
        Config::default().baudrate(115200.bps()),
        clocks,
    ).unwrap();

    rprintln!("Hello, world!"); 

    let mut value: u16 = 0;

    let pin_analog = gpioa.pa0.into_analog();

    let mut adc = Adc::adc1(device.ADC1, true, AdcConfig::default());

    let mut sample;
    loop {
        sample = adc.convert(&pin_analog, SampleTime::Cycles_480);
        // led.toggle();
        // rprintln!("value: {:02}\r", sample); 
        // writeln!(tx, "value: {:02}\r", value).unwrap();
        tx.bwrite_all(&sample.to_ne_bytes()[..]);
        // value = (value + 1) % 128;
        delay.delay_ms(10u16);
    }
}
