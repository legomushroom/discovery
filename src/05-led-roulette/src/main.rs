#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{DelayMs, OutputSwitch, entry};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds) = aux5::init();

    let pin_step: u16 = 20;

    let periods_num = 2 * leds.len();
    let periods_shift = 2;
    let periods_shift_num = periods_num - periods_shift;

    // start from 2 since the firest line is shifted
    let mut period_index = periods_shift;
    loop {
        assert!(
            period_index < 16,
            "Period index could not be larger than 15",
        );
    
        let mut led_index = 0;
        while led_index < leds.len() {
            let led = &mut leds[led_index];
            
            // 2 + 0 = 2
            // 2 + 14 = 0
            // 2 + 28 = 14
            // 2 + 42 = 12
            // 2 + 56 = 10
            // etc
            let current_led_val = (period_index + (led_index * periods_shift_num)) % periods_num;

            led_index += 1;

            if current_led_val < 3 {
                led.on().ok();
                continue;
            }

            led.off().ok();
            continue;
        }

        delay.delay_ms(pin_step);
        period_index += 1;
        period_index = period_index % periods_num;
    }
}
