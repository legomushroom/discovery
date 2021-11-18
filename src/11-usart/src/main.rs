#![no_main]
#![no_std]

use aux11::{usart1::RegisterBlock};
#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

use heapless::Vec;
use usart::{write_arr, write_str};

fn echo_serial(
    usart1: &RegisterBlock,
    is_reversed: bool,
) -> ! {
    let mut buff: Vec<u32, 128> = Vec::new();

    loop {
        while usart1.isr.read().rxne().bit_is_clear() {}

        let byte = usart1
            .rdr
            .read()
            .bits();
        
        // if not enter, save th byte to the buffer
        if byte != 13u32 {
            buff
                .push(byte)
                .expect("Failed to push to the buffer.");
            
            continue;
        }

        // enter pressed, echo the buffer

        // add new line to mimic enter key
        buff
            .push(b'\n' as u32)
            .expect("Failed to push to the buffer.");

        buff
            .push(b'\r' as u32)
            .expect("Failed to push to the buffer.");

        if is_reversed {
            buff.reverse();
        }

        write_arr(usart1, &buff[..]);

        buff.clear();
    }
}

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();

    // write_str(usart1, "\n\rThe quick brown fox jumps over the lazy dog.");

    // echo_serial(&usart1, true);

    write_str(usart1, "sdaadd");

    loop {}
}
