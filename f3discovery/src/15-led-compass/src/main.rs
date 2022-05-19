#![no_main]
#![no_std]

#[allow(unused_extern_crates)] //  bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

use core::f32::consts::PI;

use hal::{i2c::I2c, pac::Peripherals, serial::Serial, time::rate::Hertz};
use stm32f3xx_hal::{self as hal, delay::Delay, prelude::*};

// #[allow(unused_imports)]
// use aux111::{iprint, iprintln};

#[allow(unused_imports)]
use m::Float;

use lsm303agr::Lsm303agr;

use cortex_m_rt::entry;

mod leds;
pub use leds::LEDs;

mod compass_direction;
pub use compass_direction::CompassDirection;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();

    // let (usart1, _mono_timer, _itm) = aux111::init();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(cp.SYST, clocks);

    let (tx, rx) = match () {
        #[cfg(feature = "adapter")]
        () => {
            let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

            let tx = gpioa.pa9.into_af7_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
            let rx = gpioa.pa10.into_af7_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);

            (tx, rx)
        }
        #[cfg(not(feature = "adapter"))]
        () => {
            let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);

            let tx = gpioc.pc4.into_af7_push_pull(&mut gpioc.moder, &mut gpioc.otyper, &mut gpioc.afrl);
            let rx = gpioc.pc5.into_af7_push_pull(&mut gpioc.moder, &mut gpioc.otyper, &mut gpioc.afrl);

            (tx, rx)
        }
    };

    Serial::new(dp.USART1, (tx, rx), 115_200.Bd(), clocks, &mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    // clock line of the I2C bus
    let scl = gpiob.pb6.into_af4_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    // data line of the I2C bus
    let sda = gpiob.pb7.into_af4_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);

    let i2c = I2c::new(dp.I2C1, (scl, sda), Hertz::new(400_000), clocks, &mut rcc.apb1);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);

    sensor.init().unwrap();
    sensor.set_mag_odr(lsm303agr::MagOutputDataRate::Hz50).unwrap();
    
    let maybe_sensor = sensor.into_mag_continuous();

    // let compass_direction_str = "\r\nCodespaces are dool! ";

    // for c in compass_direction_str.chars() {
    //     while usart1.isr.read().txe().bit_is_clear() {} // <- NEW!

    //     // Send a single character
    //     usart1
    //         .tdr
    //         .write(|w| w.tdr().bits(c as u16));
    // }
        
    let mut leds = LEDs::new(dp.GPIOE.split(
        &mut rcc.ahb),
    );

    match maybe_sensor {
        Ok(mut sensor) => {
            loop {
                let data = sensor.mag_data()
                    .expect("Reading not found.");

                let theta = -(data.y as f32).atan2(data.x as f32); // in radians
                
                let compass_direction = if theta < -7. * PI / 8. {
                    CompassDirection::North
                } else if theta < -5. * PI / 8. {
                    CompassDirection::NorthWest
                } else if theta < -3. * PI / 8. {
                    CompassDirection::West
                } else if theta < -PI / 8. {
                    CompassDirection::SouthWest
                } else if theta < PI / 8. {
                    CompassDirection::South
                } else if theta < 3. * PI / 8. {
                    CompassDirection::SouthEast
                } else if theta < 5. * PI / 8. {
                    CompassDirection::East
                } else if theta < 7. * PI / 8. {
                    CompassDirection::NorthEast
                } else {
                    panic!("Invalid direction!");
                };

                leds.set_direction_led(compass_direction);

                delay.delay_ms(100_u16);
            }
        },
        Err(_) => {
            panic!("error");
        },
    }
}
