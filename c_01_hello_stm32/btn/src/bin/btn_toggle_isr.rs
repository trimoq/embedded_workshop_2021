#![no_main]
#![no_std]

use core::{cell::RefCell, sync::atomic::{Ordering, AtomicU16}};

use cortex_m::{interrupt::Mutex, asm::wfi, prelude::_embedded_hal_digital_ToggleableOutputPin};
use cortex_m_rt::entry;
use panic_probe as _;

use stm32f4xx_hal::{delay::Delay, prelude::*, pac, interrupt};

use btn::*;

static G_BUTTON: Mutex<RefCell<Option<BtnPin>>> = Mutex::new(RefCell::new(None));
// TODO define a global static for the led

#[entry]
fn main() -> ! {

    let mut setup = setup_board_minimal().unwrap();

    let mut led = setup.gpioa.pa5.into_push_pull_output();
    let button = Button::new();

    cortex_m::interrupt::free(|cs| {
        button.init(&cs, &mut setup.exti, setup.gpioc, &mut setup.syscfg);
        *G_BUTTON.borrow(cs).borrow_mut() = Some(button.into_inner(&cs).unwrap());
        // TODO move the led to a global static
    });


    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::EXTI15_10);
    }    

    loop {
        wfi();
    }
}

#[interrupt]
fn EXTI15_10() {
    static mut BUTTON: Option<BtnPin> = None;
    let button = steal!(BUTTON, G_BUTTON);

    // TODO move the led to a scoped static mut

    // TODO clear the interrupt pending bit on the button

    // TODO toggle the led
}

