#![no_main]
#![no_std]



use cortex_m_rt::entry;
use panic_probe as _;

use stm32f4xx_hal::{delay::Delay, prelude::*, pac, interrupt };
use rtt_target::{rprintln};

use btn::*;

static G_BUTTON: Button = Button::new();


#[entry]
fn main() -> ! {

    let mut setup = setup_board().unwrap();

    rprintln!("Setup ok");    

    // panic!("Test");

    cortex_m::interrupt::free(|cs| {
        G_BUTTON.init(&cs, &mut setup.exti, setup.gpioc, &mut setup.syscfg);
    });

    let mut delay = Delay::new(setup.core.SYST, &setup.clocks);

    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::EXTI15_10);
    }    

    loop {
        delay.delay_ms(200u8);
    }
}

#[interrupt]
fn EXTI15_10() {

    static mut ctr : u8 = 0 ;

    cortex_m::interrupt::free(|cs| {
        G_BUTTON.clear_interrupt_pending_bit(&cs);
    });  
    
    *ctr+=1;
    rprintln!("ISR ctr {}",ctr);    
}
