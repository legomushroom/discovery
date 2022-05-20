#![no_main]
#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust53964
extern crate panic_itm; // panic handler

#[allow(unused_imports)]
use stm32f3_discovery::stm32f3xx_hal::prelude::*;

#[allow(unused_imports)]
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // let (usart1, _mono_timer, _itm) = aux11::init();

    // let str = "\r\nThe quick brown fox jumps over the lazy dog.";

    // for c in str.chars() {
    //     while usart1.isr.read().txe().bit_is_clear() {}

    //     // Send a single character
    //     usart1
    //         .tdr
    //         .write(|w| w.tdr().bits(c as u16));
    // }

    loop {}
}
