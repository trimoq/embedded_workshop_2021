#![no_std]
#![no_main]

use panic_halt as _;


#[rtic::app(device = stm32f4xx_hal::pac)]
mod app {
    use core::sync::atomic::{self, Ordering};

    use stm32f4xx_hal::{
        gpio::{gpioa::PA5, Output, PushPull},
    };
    use hello_rtic::{setup_board, Button};
    use rtt_target::{rtt_init_print, rprintln};


    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        button: Button,
        led: PA5<Output<PushPull>>,
        ctr: u32
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();

        rprintln!("RTIC init");

        let device = ctx.device;
        let core = ctx.core;

        let mut setup = setup_board(device, core).unwrap();

        let button = Button::new(&mut setup.exti, setup.gpioc, &mut setup.syscfg);

        let led = setup.gpioa.pa5.into_push_pull_output();

        (Shared {}, Local { button, led , ctr: 0u32}, init::Monotonics())
    }



    #[task(binds = EXTI15_10, local = [button, led,ctr])]
    fn button_click(ctx: button_click::Context) {
        rprintln!("BTN ISR count: {}",ctx.local.ctr);
        *ctx.local.ctr +=1;
        ctx.local.button.clear_interrupt_pending_bit();
        ctx.local.led.toggle();
    }

    /**
     * Can't have rtt if using default wfi(), therefore busyloop for now
     * 
     */
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            atomic::compiler_fence(Ordering::SeqCst);
        }
    }
}