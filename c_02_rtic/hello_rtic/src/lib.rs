#![no_std]

use core::{cell::RefCell, sync::atomic::{Ordering, AtomicBool}};

use cortex_m::{interrupt::{Mutex, CriticalSection}};

use stm32f4xx_hal::{gpio::{Edge, gpioa::{self, PA5}, gpioc::{self, PC13, Parts}, Input, PullUp, Output, PushPull,},  pac::{self, EXTI, }, prelude::*, syscfg::SysCfg, rcc::{Clocks, Rcc}};
use rtt_target::{rtt_init_print};

pub struct Setup{
    pub clocks: Clocks,
    pub syscfg: SysCfg,
    pub exti: EXTI,
    pub gpioa: gpioa::Parts,
    pub gpioc: gpioc::Parts
}

static BOARD_INITIALIZED: AtomicBool = AtomicBool::new(false);

/**
 * The Pin definition for the Button connected to GPIO-C-13
 */
pub type BtnPin = PC13<Input<PullUp>>;

/**
 * The Pin definition for the on-board LED connected to GPIO-A-5
 */
pub type LedPin = PA5<Output<PushPull>>;


pub fn setup_board(
    device: pac::Peripherals,
    mut _core: cortex_m::Peripherals
) -> Result<Setup,()>{
    if BOARD_INITIALIZED.load(Ordering::Relaxed) {
        return Err(())
    }

    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr
        .sysclk(84.mhz())
        // .use_hse(8.mhz())
        // .hclk(8.mhz())   
        // .pclk1(8.mhz())
        // .pclk2(8.mhz())
        // .adcclk(8.mhz())
        .freeze();
    let syscfg = device.SYSCFG.constrain();
    let gpioa = device.GPIOA.split();
    let gpioc = device.GPIOC.split();
    let exti = device.EXTI;
    BOARD_INITIALIZED.store(true, Ordering::Relaxed);
    Ok(Setup{ clocks, syscfg, exti, gpioa, gpioc})
}

pub struct Button{
    pin : BtnPin,
}

impl Button{
    pub fn new( 
        exti: &mut EXTI, 
        gpioc: Parts,           // we take ownership of the gpio register, can't initialize another button here
        syscfg: &mut SysCfg
    ) -> Self{
        let mut pin = gpioc.pc13.into_pull_up_input();
        pin.make_interrupt_source(syscfg);
        pin.enable_interrupt(exti );
        pin.trigger_on_edge( exti, Edge::Rising);
        
        Button{
            pin,
        }
    }    

    /**
     * Clears the interrupt pending bit if the button was initialized
     */
    pub fn clear_interrupt_pending_bit( &mut self){
        self.pin.clear_interrupt_pending_bit();
    }
}
