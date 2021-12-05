#![no_std]

use core::{cell::RefCell, sync::atomic::{Ordering, AtomicBool}};

use cortex_m::{interrupt::{Mutex, CriticalSection}};

use stm32f4xx_hal::{gpio::{Edge, gpioa::{self, PA5}, gpioc::{self, PC13, Parts}, Input, PullUp, Output, PushPull,},  pac::{self, EXTI, }, prelude::*, syscfg::SysCfg, rcc::{Clocks, Rcc}};
use rtt_target::{rtt_init_print};

pub struct Setup{
    pub core: cortex_m::Peripherals,
    pub clocks: Clocks,
    pub syscfg: SysCfg,
    pub exti: EXTI,
    pub gpioa: gpioa::Parts,
    pub gpioc: gpioc::Parts
}

static BOARD_INITIALIZED: AtomicBool = AtomicBool::new(false);


// Set up the board with rtt
pub fn setup_board() -> Result<Setup,()>{
    rtt_init_print!();
    setup_board_minimal()    
}

// Set up the board without any fluff
pub fn setup_board_minimal() -> Result<Setup,()>{
    if BOARD_INITIALIZED.load(Ordering::Relaxed) {
        return Err(())
    }

    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();
    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();
    let syscfg = device.SYSCFG.constrain();
    let gpioa = device.GPIOA.split();
    let gpioc = device.GPIOC.split();
    let exti = device.EXTI;
    BOARD_INITIALIZED.store(true, Ordering::Relaxed);
    Ok(Setup{ core, clocks, syscfg, exti, gpioa, gpioc})
}

/**
 * The Pin definition for the Button connected to GPIO-C-13
 */
pub type BtnPin = PC13<Input<PullUp>>;

/**
 * The Pin definition for the on-board LED connected to GPIO-A-5
 */
pub type LedPin = PA5<Output<PushPull>>;


pub struct Button{
    pin : Mutex<RefCell<Option<BtnPin>>>,
    initalized: AtomicBool
}

impl Button{
    pub const fn new() -> Self{
        Button{
            pin: Mutex::new(RefCell::new(None)),
            initalized: AtomicBool::new(false)
        }
    }

    /**
     * Return the inner pin and consumes the button
     */
    pub fn into_inner(self, cs: &CriticalSection)->Option<PC13<Input<PullUp>>>{
        self.pin.borrow(cs).borrow_mut().take()
    }

    /**
     * Initializes the button, takes care of interrupt source and such
     */
    pub fn init(&self, cs: &CriticalSection,
        exti: &mut EXTI, 
        gpioc: Parts, 
        syscfg: &mut SysCfg
    ){
        if self.initalized.load(Ordering::Relaxed){
            return;
        }
        let mut pin = gpioc.pc13.into_pull_up_input();
        pin.make_interrupt_source(syscfg);
        pin.enable_interrupt(exti );
        pin.trigger_on_edge( exti, Edge::Rising);
        self.pin.borrow(cs).replace(Some(pin));
        self.initalized.store(true, Ordering::Relaxed)
    }

    /**
     * Clears the interrupt pending bit if the button was initialized
     */
    pub fn clear_interrupt_pending_bit( &self, cs: &CriticalSection){
        if !self.initalized.load(Ordering::Relaxed){
            panic!("Cannot use button before initialization");
        }
        self.pin.borrow(cs).borrow_mut().as_mut().unwrap().clear_interrupt_pending_bit();
    }
}


/**
 * If the static $source is empty, we steal fom the global variable $target and insert it
 */
#[macro_export]
macro_rules! steal {
    ($source:ident, $target:ident) => {
        $source.get_or_insert_with(||{
            cortex_m::interrupt::free(|cs| {
                $target.borrow(cs).replace(None).unwrap()
            })
        })
    };
}