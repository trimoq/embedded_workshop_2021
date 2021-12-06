#![no_std]
#![no_main]

use panic_halt as _;


#[rtic::app(device = stm32f4xx_hal::pac)]
mod app {

    use stm32f4xx_hal::{
        gpio::{gpioa::PA5, Output, PushPull},
        prelude::*,
        timer::{Timer,CountDownTimer, Event}, pac::TIM2
    };


    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: PA5<Output<PushPull>>,
        timer: CountDownTimer<TIM2>
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {

        let device = ctx.device;

        let rcc = device.RCC.constrain();
        let clocks = rcc.cfgr
            .sysclk(84.mhz())
            .freeze();
        let _syscfg = device.SYSCFG.constrain();

        let led = device.GPIOA.split().pa5.into_push_pull_output();

        let mut timer = Timer::new(device.TIM2, &clocks).start_count_down(20.hz());
        timer.listen(Event::TimeOut);

        (Shared {}, Local {led, timer}, init::Monotonics())
    }



    #[task(binds = TIM2, local = [led, timer])]
    fn button_click(ctx: button_click::Context) {
        ctx.local.led.toggle();
        ctx.local.timer.clear_interrupt(Event::TimeOut);
    }   
}