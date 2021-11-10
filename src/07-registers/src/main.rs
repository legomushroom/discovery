#![no_main]
#![no_std]

// use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln, ITM, RegisterBlock};

const GPIOE: u32 = 0x48001800;
const BSRR: u8 = 0x18;
const ODR: u8 = 0x14;

const GPIOE_BSRR: u32 = GPIOE | BSRR as u32;
const GPIOE_ODR: u32 = GPIOE | ODR as u32;

// // Print the current contents of odr
// fn iprint_odr(itm: &mut ITM) {
//     unsafe {
//         iprintln!(
//             &mut itm.stim[0],
//             "ODR = 0x{:04x}",
//             ptr::read_volatile(GPIOE_ODR as *const u16)
//         );
//     }
// }

#[entry]
fn main() -> ! {
    // aux7::init();
    let gpioe = aux7::init().1;

    gpioe.bsrr.write(|w| {
        return w
            .bs9().set_bit()
            .bs10().set_bit()
            .bs11().set_bit();
    });

    gpioe.bsrr.write(|w| {
        return w;
    });

    // unsafe {
    //     // Print the initial contents of ODR
    //     // iprint_odr(&mut itm);

    //     // Turn on the "North" LED (red)
    //     ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);
    //     // iprint_odr(&mut itm);

    //     ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 10);
    //     // iprint_odr(&mut itm);

    //     // // Turn on the "East" LED (green)
    //     ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);
    //     // iprint_odr(&mut itm);

    //     // // Turn off the "North" LED
    //     ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));
    //     // iprint_odr(&mut itm);

    //     // // Turn off the "East" LED
    //     ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
    //     // iprint_odr(&mut itm);

    //     ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (10 + 16));
    //     // iprint_odr(&mut itm);
    // }

    loop {}
}
