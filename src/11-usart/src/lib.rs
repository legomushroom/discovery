#![no_std]

use aux11::{usart1::RegisterBlock};
#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

pub fn write_str<T: AsRef<str>>(
    usart1: &RegisterBlock,
    str: T,
) {
    let str = str.as_ref();
    for char in str.chars() {
        while usart1.isr.read().txe().bit_is_clear() {}

        // Send a single character
        usart1
            .tdr
            .write(|w| {
                return w.tdr().bits(char as u16);
            });
    }
}

pub fn write_arr<>(
    usart1: &RegisterBlock,
    arr: &[u32],
) {
    for byte in arr {
        while usart1.isr.read().txe().bit_is_clear() {}

        // Send a single character
        usart1
            .tdr
            .write(|w| {
                return w.tdr().bits(*byte as u16);
            });
    }
}