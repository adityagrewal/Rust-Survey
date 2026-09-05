#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use lpc8xx_hal::{cortex_m::peripheral::Peripherals as CorePeripherals, prelude::*, delay::Delay};
use lpc8xx_hal::pac as pac;

#[entry]
fn main() -> ! {
    let cp = CorePeripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST);

    // SWM is needed to enable the DACOUT0 fixed-pin function, and is not
    // clock-gated on by default on the LPC845
    dp.SYSCON
        .sysahbclkctrl0
        .modify(|_, w| w.iocon().set_bit().swm().set_bit().dac0().set_bit());
    
    // power up the DAC: the PDRUNCFG bits are power-DOWN bits, so clear it
    dp.SYSCON.pdruncfg.modify(|_, w| w.dac0().clear_bit());
    
    // route the analog output to PIO0_17. PINENABLE0 bits are active low:
    // clearing the bit enables the fixed-pin function.
    dp.SWM0.pinenable0.modify(|_, w| w.dacout0().clear_bit());
    
    // DACMODE also disables the pin's digital input buffer
    dp.IOCON.pio0_17.modify(|_, w| { w.dacmode().set_bit() });

    let mut level: u16 = 0;
    let mut rising = true;
    loop {
        dp.DAC0.cr.write(|w| unsafe { w.value().bits(level) });

        if rising {
            if level >= 1020 {
                rising = false;
            } else {
                level += 4;
            }
        } else {
            if level == 0 {
                rising = true;
            } else {
                level -= 4;
            }
        }
        delay.delay_ms(5u32);
    }
}