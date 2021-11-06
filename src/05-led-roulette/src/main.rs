#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{DelayMs, OutputSwitch, entry};

// delay amount on each step/period
const STEP_DELAY_MS: u16 = 20;
// how long does each LED stays ON
const ON_STATE_PERIOD_LEN: usize = 3;
// number of period the next LED shifted compared to the current LED
const PERIOD_SHIFT: usize = 2;

#[entry]
fn main() -> ! {
    let (mut delay, mut leds) = aux5::init();

    // get total number of periods
    let periods_num = 2 * leds.len();
    // size of the OFF period for each LED (e.g. 14 for diagramm in the book)
    let periods_shift_num = periods_num - PERIOD_SHIFT;

    // start from 2 since the firest line is shifted
    let mut period_index = PERIOD_SHIFT;
    loop {
        assert!(
            period_index < periods_num,
            "Period index could not be larger than total number of periods.",
        );

        // set state of the each LED for the current period(`period_index`)
        let mut led_index = 0;
        while led_index < leds.len() {
            // get current LED preference
            let led = &mut leds[led_index];
            
            // 2 + 0 % 16 = 2
            // 2 + 14 % 16 = 0
            // 2 + 28 % 16 = 14
            // 2 + 42 % 16 = 12
            // 2 + 56 % 16 = 10
            // etc
            let current_led_val = (period_index + (led_index * periods_shift_num)) % periods_num;

            // increment the LED index for the next iteration
            led_index += 1;

            // if inside the "on" period, turn the LED `on`
            if current_led_val < ON_STATE_PERIOD_LEN {
                led.on().ok();
            } else {
                // otherwise, turn the LED `off`
                led.off().ok();
            }

            continue;
        }

        // wait for a bit before going into the next period
        delay.delay_ms(STEP_DELAY_MS);
        
        // increment the period number
        period_index += 1;
        // clamp the period number to the (periods_num - 1) max
        period_index = period_index % periods_num;
    }
}
