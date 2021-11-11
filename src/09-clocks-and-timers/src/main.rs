#![no_main]
#![no_std]

use core::convert::TryInto;

use aux9::{entry, switch_hal::OutputSwitch, tim6};

// The registers we'll be using in this section are:
//
// - SR, the status register.
// - EGR, the event generation register.
// - CNT, the counter register.
// - PSC, the prescaler register.
// - ARR, the autoreload register.
//
// We'll be using the timer as a one-shot timer. It will sort of work like an alarm clock. We'll set the timer to go off after some amount of time and then we'll wait until the timer goes off. The documentation refers to this mode of operation as one pulse mode.
// Here's a description of how a basic timer works when configured in one pulse mode:
// - The counter is enabled by the user (CR1.CEN = 1).
// - The CNT register resets its value to zero and, on each tick, its value gets incremented by one.
// - Once the CNT register has reached the value of the ARR register, the counter will be disabled by hardware (CR1.CEN = 0) and an update event will be raised (SR.UIF = 1).
//
// TIM6 is driven by the APB1 clock, whose frequency doesn't have to necessarily match the processor frequency. That is, the APB1 clock could be running faster or slower. The default, however, is that both APB1 and the processor are clocked at 8 MHz.
//
// The tick mentioned in the functional description of the one pulse mode is not the same as one tick of the APB1 clock. The CNT register increases at a frequency of apb1 / (psc + 1) times per second, where apb1 is the frequency of the APB1 clock and psc is the value of the prescaler register, PSC.

const KHZ: u16 = 1000; // 1 KHz
const MHZ: u32 = 1000 * KHZ as u32; // 1 MHz
const CLOCK_FREQ: u32 = 8 * MHZ; // Abn clock frequency is 8 MHz

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    // ARR: write how many ticks to wait
    tim6.arr.write(|w| {
        return w.arr().bits(ms);
    });

    // SR: clear the event bit
    tim6.sr.modify(|_, w| {
        return w.uif().clear_bit();
    });

    // CEN: enable the counter
    tim6.cr1.modify(|_, w| {
        return w.cen().set_bit();
    });
    
    // SR: busy waiting: wait until the update event occurs
    while !tim6.sr.read().uif().bit_is_set() {}
}

#[entry]
fn main() -> ! {
    let (leds, rcc, tim6) = aux9::init();
    let mut leds = leds.into_array();

    // power up TIM6 timer
    rcc.apb1enr.modify(|_, w| {
        return w.tim6en().set_bit();
    });

    // OPM select one pulse mode
    // CEN keep counter disabled for now
    tim6.cr1.write(|w| {
        return w
            .opm().set_bit()
            .cen().clear_bit();
    });

    // PSC: configure TIM6 prescaler to make the time to operate at 1 KHz
    tim6.psc.write(|w| {
        // 8 MHz (apb clock) / 8 KHz (psc) = 1000, e.g. 1000 ticks per second or 1 tick per ms
        let psc: u16 = ((CLOCK_FREQ / 1000 as u32))
            .try_into().expect("Cannot get terget value of the TIM6 prescaler register."); 

        return w
            .psc()
            .bits(psc - 1);
    });

    let ms = 50;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().unwrap();
            delay(tim6, ms);
            leds[curr].off().unwrap();
            delay(tim6, ms);
        }
    }
}
