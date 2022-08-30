#![no_std]
#![no_main]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust53964
extern crate panic_itm; // panic handler

#[allow(unused_imports)]
use stm32f3_discovery::stm32f3xx_hal::prelude::*;

#[allow(unused_imports)]
use cortex_m_rt::entry;

use stm32f3_discovery::stm32f3xx_hal::pac::{usart1, USART1};

#[entry]
fn main() -> ! {
    let usart1: &'static mut usart1::RegisterBlock = unsafe {
        &mut *(USART1::ptr() as *mut _)
    };

    let str = "\r\nThe quick brown fox jumps over the lazy dog.";

    for c in str.chars().into_iter() {
        while usart1.isr.read().txe().bit_is_clear() {}

        // Send a single character
        usart1
            .tdr
            .write(|w| w.tdr().bits(c as u16));
    }

    loop {}
}
