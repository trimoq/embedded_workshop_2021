#![no_std]
#![no_main]

use panic_halt as _;


#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [USART1])]
mod app {
    use fugit::ExtU32;
    use core::sync::atomic::{self, Ordering};

    use stm32f4xx_hal::{
        gpio::{gpioa::PA5, Output, PushPull},
        prelude::*
    };
    use rtt_target::{rtt_init_print, rprintln};
    use dwt_systick_monotonic::DwtSystick;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: PA5<Output<PushPull>>
    }

    // We run at 84Mhz
    const MONO_HZ : u32 = 84_000_000;
    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<MONO_HZ>; 


    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();

        rprintln!("RTIC init");

        let rcc = ctx.device.RCC.constrain();
        let _clocks = rcc.cfgr.sysclk(84.mhz()).freeze();

        let led = ctx.device.GPIOA.split().pa5.into_push_pull_output();

        // Debug Control Block and Data Watchpoint and Trace unit as hacks
        let mut dcb = ctx.core.DCB;
        let dwt = ctx.core.DWT;
        let systick = ctx.core.SYST;

        // This is a hack
        let mono = DwtSystick::new(&mut dcb, dwt, systick, MONO_HZ);
        
        toggle_task::spawn().ok();
        (Shared {}, Local { led }, init::Monotonics(mono))
    }



    #[task(local = [led])]
    fn toggle_task(ctx: toggle_task::Context) {
        rprintln!("RTIC toggle");
        toggle_task::spawn_after(1.secs()).ok();
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