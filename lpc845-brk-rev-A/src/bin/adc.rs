#![no_std]
#![no_main]
use panic_halt as _;
use cortex_m_rt::entry;
use lpc8xx_hal::{cortex_m::peripheral::Peripherals as CorePeripherals, prelude::*, delay::Delay, gpio::Level, Peripherals};
use lpc8xx_hal::syscon::clock_source::AdcClock;
use nb::block;

#[entry]
fn main() -> ! {
    let cp = CorePeripherals::take().unwrap();
    let p = Peripherals::take().unwrap();

    let mut syscon = p.SYSCON.split();
    let swm = p.SWM.split();
    let mut swm_handle = swm.handle.enable(&mut syscon.handle);
    let gpio = p.GPIO.enable(&mut syscon.handle);

    // active low: High is LED off
    let mut led = p.pins.pio1_2.into_output_pin(gpio.tokens.pio1_2, Level::High);
    let mut delay = Delay::new(cp.SYST);

    let adc_clock = AdcClock::new_default();
    let mut adc = p.ADC.enable(&adc_clock, &mut syscon.handle);

    let (mut adc_pin, _) = swm
        .fixed_functions
        .adc_0
        .assign(p.pins.pio0_7.into_swm_pin(), &mut swm_handle);

    loop {
        let value: u16 = block!(adc.read(&mut adc_pin)).unwrap();
        defmt::info!("adc: {}", value);   // <- add this line

        let period_ms = 20 + (value as u32 * 500) / 4095;
        led.set_low();
        delay.delay_ms(period_ms);
        led.set_high();
        delay.delay_ms(period_ms);
    }
}
