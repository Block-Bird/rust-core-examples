#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rtic::app;
use stm32f3::stm32f303;
use panic_halt as _; // panic handler

#[app(device = stm32f303, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    #[init(schedule = [blink])]
    fn init(cx: init::Context) {
        let mut rcc = cx.device.RCC;
        let mut gpioe = cx.device.GPIOE;
        let core = cx.core;

        // Enable the peripheral clock for GPIOE
        rcc.ahbenr.modify(|_, w| w.iopcen().set_bit());

        // Set the 9th bit of GPIOE_MODER to 1 (output mode)
        gpioe.moder.modify(|_, w| w.moder9().output());

        // Start the SysTick counter to fire the TICK interrupt every 1 ms
        core.SYST.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
        core.SYST.set_reload(8_000_000); // 1 ms
        core.SYST.enable_counter();
        core.SYST.enable_interrupt();
    }

    #[task(schedule = [blink], resources = [gpioe])]
    fn blink(cx: blink::Context) {
        // Toggle the 9th pin
        cx.resources.gpioe.odr.modify(|_, w| w.odr9().bit(!w.odr9().bit()));

        // Schedule the next blink in 500 ms
        cx.schedule.blink(cx.scheduled + 500_000.cycles()).unwrap();
    }

    extern "C" {
        fn SysTick();
    }
};
