#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux8::entry;

#[entry]
fn main() -> ! {
    let (gpioe, rcc) = aux8::init();

    // initialize GPIOE peripheral
    rcc.ahbenr.write(|w| {
        return w
            .iopeen()
            .set_bit();
    });

    // set the LED pins into the Output mode
    gpioe.moder.write(|w| {
        return w
            .moder8().output()
            .moder9().output()
            .moder10().output()
            .moder11().output()
            .moder12().output()
            .moder13().output()
            .moder14().output()
            .moder15().output();
    });

    // Turn on all the LEDs in the compass
    gpioe.odr.write(|w| {
        return w
            .odr8().set_bit()
            .odr9().set_bit()
            .odr10().set_bit()
            .odr11().set_bit()
            .odr12().set_bit()
            .odr13().set_bit()
            .odr14().set_bit()
            .odr15().set_bit();
    });

    aux8::bkpt();

    loop {}
}
