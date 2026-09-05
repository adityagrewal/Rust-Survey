#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use lpc8xx_hal::{cortex_m::peripheral::Peripherals as CorePeripherals, prelude::*, delay::Delay, gpio::Level, Peripherals};

#[entry]
fn main() -> ! {
    let cp = CorePeripherals::take().unwrap();
    let p = Peripherals::take().unwrap();

    let mut syscon = p.SYSCON.split();
    let gpio = p.GPIO.enable(&mut syscon.handle);
    let pins = p.pins;

    // LED is active low, button is active low with an on-board pull-up
    let mut led = pins.pio1_2.into_output_pin(gpio.tokens.pio1_2, Level::High);
    let button = pins.pio0_4.into_input_pin(gpio.tokens.pio0_4);
    let mut delay = Delay::new(cp.SYST);

    loop {
        if button.is_low() {
            led.set_low();   // pressed: LED on
        } else {
            led.set_high();  // released: LED off
        }
        delay.delay_ms(20u32); // simple software debounce
    }
}
