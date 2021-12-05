#![no_main]
#![no_std]

use core::{cell::RefCell, sync::atomic::{Ordering, AtomicU16}};

use cortex_m::{interrupt::Mutex, asm::wfi, prelude::_embedded_hal_digital_ToggleableOutputPin};
use cortex_m_rt::entry;
use panic_probe as _;

use stm32f4xx_hal::{delay::Delay, prelude::*, pac, interrupt};

use btn::*;

static G_BUTTON: Mutex<RefCell<Option<BtnPin>>> = Mutex::new(RefCell::new(None));
static G_LED: Mutex<RefCell<Option<LedPin>>> = Mutex::new(RefCell::new(None));


#[entry]
fn main() -> ! {

    let mut setup = setup_board_minimal().unwrap();

    let mut led = setup.gpioa.pa5.into_push_pull_output();
    let button = Button::new();

    cortex_m::interrupt::free(|cs| {
        button.init(&cs, &mut setup.exti, setup.gpioc, &mut setup.syscfg);
        *G_BUTTON.borrow(cs).borrow_mut() = Some(button.into_inner(&cs).unwrap());
        *G_LED.borrow(cs).borrow_mut() = Some(led);
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
    static mut LED: Option<LedPin> = None;

    let button = steal!(BUTTON, G_BUTTON);
    let led = steal!(LED, G_LED);

    cortex_m::interrupt::free(|_cs| {
        button.clear_interrupt_pending_bit();
    });
    led.toggle();
}

