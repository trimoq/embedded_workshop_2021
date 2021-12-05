#![no_main]
#![no_std]



use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use panic_probe as _;

use stm32f4xx_hal::{delay::Delay, prelude::*, pac, interrupt };
use rtt_target::{rprintln};

use btn::*;

static G_BUTTON: Mutex<RefCell<Option<BtnPin>>> = Mutex::new(RefCell::new(None));


#[entry]
fn main() -> ! {

    let mut setup = setup_board().unwrap();

    rprintln!("Setup ok");    

    cortex_m::interrupt::free(|cs| {
        let button = Button::new();
        button.init(&cs, &mut setup.exti, setup.gpioc, &mut setup.syscfg);
        *G_BUTTON.borrow(cs).borrow_mut() = Some(button.into_inner(&cs).unwrap())
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

    static mut CTR : u8 = 0 ;
    static mut BUTTON: Option<BtnPin> = None;

    let button = steal!(BUTTON, G_BUTTON);

    cortex_m::interrupt::free(|_cs| {
        button.clear_interrupt_pending_bit();
    });
    
    *CTR+=1;
    rprintln!("ISR ctr {}",CTR);    
}

