#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [EXTI0])]
mod app {
    use core::sync::atomic::{self, Ordering};

    use rtt_target::{rtt_init_print, rprintln};
    use dwt_systick_monotonic::DwtSystick;
    pub use fugit::ExtU32;

    use stm32f4xx_hal::{
        adc::{
            config::{AdcConfig,  SampleTime, Scan, Sequence, Eoc, Clock},
            Adc, 
        },
        pac::{self, ADC1},
        prelude::*, gpio::{gpioa::PA5, Output, PushPull},
    };

    const MONO_HZ: u32 = 84_000_000; // 84 MHz

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<MONO_HZ>;


    #[shared]
    struct Shared {
        adc: Adc<ADC1>,
    }

    #[local]
    struct Local {
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("RTIC init");

        let device: pac::Peripherals = ctx.device;

        let rcc = device.RCC.constrain();
        let _clocks = rcc
            .cfgr
            .sysclk(84.mhz())
            .pclk2(84.khz())
            .freeze();

        let mut dcb = ctx.core.DCB;
        let dwt = ctx.core.DWT;
        let systick = ctx.core.SYST;

        let mono = DwtSystick::new(&mut dcb, dwt, systick, MONO_HZ);

        let gpioa = device.GPIOA.split();
        let voltage = gpioa.pa0.into_analog();

        let adc_config = AdcConfig::default()
            .end_of_conversion_interrupt(Eoc::Conversion)
            .scan(Scan::Enabled)
            .clock(Clock::Pclk2_div_8);

        
        // TODO setup led

        let mut adc = Adc::adc1(device.ADC1, true, adc_config);
        adc.configure_channel(&voltage, Sequence::One, SampleTime::Cycles_480);

        polling::spawn_after(1.secs()).ok();

        (
            Shared { adc },
            Local {},
            init::Monotonics(mono),
        )
    }

    #[task(shared = [adc])]
    fn polling(mut ctx: polling::Context) {
        ctx.shared.adc.lock(|adc|{
            adc.start_conversion();
        });
        polling::spawn_after(10.millis()).ok();
    }

    // TODO adjust `shared` or `local` here 
    #[task(binds = ADC, shared = [adc], local = [])]
    fn adc_task(mut ctx: adc_task::Context) {
        // rprint!("adc_task triggered");
        let sample = ctx.shared.adc.lock(|adc|{
            adc.current_sample()
        });
        // TODO decide whether to toggle led
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