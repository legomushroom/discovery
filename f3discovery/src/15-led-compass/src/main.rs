#![no_main]
#![no_std]

#[allow(unused_extern_crates)] //  bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

use core::fmt::{self, Debug};
use core::fmt::Write;
use core::f32::consts::PI;

use embedded_hal::digital::v2::PinState;
use hal::gpio::marker::Gpio;
use hal::gpio::{Gpioe, Output, Pin, PushPull};
use hal::{i2c::I2c, pac::{Peripherals, USART1, usart1}, serial::Serial, time::rate::Hertz};
use stm32f3xx_hal::{self as hal, delay::Delay, prelude::*};

use m::Float;

// use embedded_hal::I2c;
use lsm303agr::Lsm303agr;

use cortex_m_rt::entry;

pub struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // write_str(self.usart1, s);

        // let str = str.as_ref();
        for char in s.chars() {
            while self.usart1.isr.read().txe().bit_is_clear() {}

            // Send a single character
            self.usart1
                .tdr
                .write(|w| {
                    return w.tdr().bits(char as u16);
                });
        }

        Ok(())
    }
}

impl SerialPort {
    pub fn new(
        usart1: &'static mut usart1::RegisterBlock,
    ) -> Self {
        return SerialPort { usart1 };
    }
}

/// Cardinal directions. Each one matches one of the user LEDs.
pub enum Direction {
    /// North / LD3
    North,
    /// Northeast / LD5
    NorthEast,
    /// East / LD7
    East,
    /// Southeast / LD9
    SouthEast,
    /// South / LD10
    South,
    /// Southwest / LD8
    SouthWest,
    /// West / LD6
    West,
    /// Northwest / LD4
    NorthWest,
}
impl Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::North => write!(f, "North"),
            Self::NorthWest => write!(f, "NorthWest"),
            Self::NorthEast => write!(f, "NorthEast"),
            Self::South => write!(f, "South"),
            Self::SouthWest => write!(f, "SouthWest"),
            Self::SouthEast => write!(f, "SouthEast"),
            Self::West => write!(f, "West"),
            Self::East => write!(f, "East"),
        }
    }
}

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();

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

    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

    let mut led8 = gpioe.pe8.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led9 = gpioe.pe9.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led10 = gpioe.pe10.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led11 = gpioe.pe11.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led12 = gpioe.pe12.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led13 = gpioe.pe13.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led14 = gpioe.pe14.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led15 = gpioe.pe15.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    match maybe_sensor {
        Ok(mut sensor) => {
            loop {
                let data = sensor.mag_data()
                    .expect("Reading not found.");

                let theta = -(data.y as f32).atan2(data.x as f32); // in radians
                
                let direction = if theta < -7. * PI / 8. {
                    Direction::North
                } else if theta < -5. * PI / 8. {
                    Direction::NorthWest
                } else if theta < -3. * PI / 8. {
                    Direction::West
                } else if theta < -PI / 8. {
                    Direction::SouthWest
                } else if theta < PI / 8. {
                    Direction::South
                } else if theta < 3. * PI / 8. {
                    Direction::SouthEast
                } else if theta < 5. * PI / 8. {
                    Direction::East
                } else if theta < 7. * PI / 8. {
                    Direction::NorthEast
                } else {
                    Direction::North
                };

                led8.set_state(PinState::Low).expect("Cannot set LED8 state.");
                led9.set_state(PinState::Low).expect("Cannot set LED9 state.");
                led10.set_state(PinState::Low).expect("Cannot set LED10 state.");
                led11.set_state(PinState::Low).expect("Cannot set LED11 state.");
                led12.set_state(PinState::Low).expect("Cannot set LED12 state.");
                led13.set_state(PinState::Low).expect("Cannot set LED13 state.");
                led14.set_state(PinState::Low).expect("Cannot set LED14 state.");
                led15.set_state(PinState::Low).expect("Cannot set LED15 state.");

                match direction {
                    Direction::South => {
                        led8.set_state(PinState::High).expect("Cannot set LED8 state.");
                    },
                    Direction::SouthWest => {
                        led9.set_state(PinState::High).expect("Cannot set LED9 state.");
                    },
                    Direction::West => {
                        led10.set_state(PinState::High).expect("Cannot set LED10 state.");
                    },
                    Direction::NorthWest => {
                        led11.set_state(PinState::High).expect("Cannot set LED11 state.");
                    },
                    Direction::North => {
                        led12.set_state(PinState::High).expect("Cannot set LED12 state.");
                    },
                    Direction::NorthEast => {
                        led13.set_state(PinState::High).expect("Cannot set LED13 state.");
                    },
                    Direction::East => {
                        led14.set_state(PinState::High).expect("Cannot set LED14 state.");
                    },
                    Direction::SouthEast => {
                        led15.set_state(PinState::High).expect("Cannot set LED15 state.");
                    },
                }

                delay.delay_ms(100_u16);
            }
        },
        Err(_) => {
            panic!("error");
        },
    }
}
