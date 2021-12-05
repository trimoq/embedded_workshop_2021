use anyhow::Result;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

const GPIO_LED: u8 = 23;    // phys 16

fn main() -> Result<()> {
    let mut pin = Gpio::new()?
        .get(GPIO_LED)?
        .into_output();
    loop {
        pin.toggle();
        thread::sleep(Duration::from_millis(200));
    }
}