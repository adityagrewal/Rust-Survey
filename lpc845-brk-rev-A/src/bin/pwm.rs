#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use lpc8xx_hal::pac::Peripherals;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
 
    dp.SYSCON.sysahbclkctrl0.modify(|_, w| w.sct().set_bit().swm().set_bit());
    dp.SWM0.pinassign7.modify(|_, w| unsafe { w.sct_out0_o().bits(33) }); // PIO1_1

    dp.SCT0.regmode.write(|w| unsafe { w.bits(0) });          // both as MATCH
    dp.SCT0.config.write(|w| w.unify().set_bit().autolimit_l().set_bit());

    dp.SCT0.capctrl_matchrel_sctmatchrel()[0].write(|w| unsafe { w.bits(24_000) });
    dp.SCT0.capctrl_matchrel_sctmatchrel()[1].write(|w| unsafe { w.bits(0) });

    dp.SCT0.event[0].ctrl.write(|w| unsafe { w.matchsel().bits(0).combmode().bits(0b01) });
     dp.SCT0.event[0].state.write(|w| unsafe { w.bits(1) });   // enable in state 0
    dp.SCT0.event[1].ctrl.write(|w| unsafe { w.matchsel().bits(1).combmode().bits(0b01) });
    dp.SCT0.event[1].state.write(|w| unsafe { w.bits(1) });

    dp.SCT0.out[0].set.write(|w| unsafe { w.bits(1 << 0) });
    dp.SCT0.out[0].clr.write(|w| unsafe { w.bits(1 << 1) });

    dp.SCT0.ctrl.modify(|_, w| w.clrctr_l().set_bit().halt_l().clear_bit());
    let mut duty: u32 = 0;
    let mut rising = true;
    loop {
        dp.SCT0.capctrl_matchrel_sctmatchrel()[1].write(|w| unsafe { w.bits(duty) });

        if rising {
            duty += 240;
            if duty == 24_000 { rising = false; }
        } else {
            duty -= 240;
            if duty == 0 { rising = true; }
        }
        cortex_m::asm::delay(300_000)
    }
}