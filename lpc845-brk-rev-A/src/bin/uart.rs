#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use lpc8xx_hal::prelude::*;
use lpc8xx_hal::{usart, Peripherals};
use nb::block;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    let mut syscon = p.SYSCON.split();
    let swm = p.SWM.split();
    let mut swm_handle = swm.handle.enable(&mut syscon.handle);

    let (u0_rxd, _) = swm.movable_functions.u0_rxd.assign(
        p.pins.pio0_24.into_swm_pin(), &mut swm_handle,
    );
    let (u0_txd, _) = swm.movable_functions.u0_txd.assign(
        p.pins.pio0_25.into_swm_pin(), &mut swm_handle,
    );

    let clock_config = usart::Clock::new_with_baudrate(115_200);
    let mut usart0 = p.USART0.enable_async(
        &clock_config,
        &mut syscon.handle,
        u0_rxd,
        u0_txd,
        usart::Settings::default(),
    );

    loop {
        let byte = block!(usart0.read()).unwrap();
        block!(usart0.write(byte)).unwrap();
    }
}
