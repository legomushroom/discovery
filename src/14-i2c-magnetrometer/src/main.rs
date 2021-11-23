#![no_main]
#![no_std]

#[allow(unused_extern_crates)] //  bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

use core::fmt::{self, Debug};
use core::fmt::Write;

use hal::{i2c::I2c, pac::{Peripherals, USART1, usart1}, serial::Serial, time::rate::Hertz};
use stm32f3xx_hal::{self as hal, delay::Delay, prelude::*};

// use embedded_hal::I2c;
use lsm303agr::Lsm303agr;

use cortex_m_rt::entry;


pub fn write_str<T: AsRef<str>>(
    usart1: &'static usart1::RegisterBlock,
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
    usart1: &'static usart1::RegisterBlock,
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

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n"), $($arg)*)
    };
}


enum Direction {
    Southeast,
    Southwest,
    Northeast,
    Northwest,
}

impl Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Southeast => write!(f, "[Southeast]"),
            Self::Southwest => write!(f, "[Southwest]"),
            Self::Northeast => write!(f, "[Northeast]"),
            Self::Northwest => write!(f, "[Northwest]"),
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
    // sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    sensor.set_mag_odr(lsm303agr::MagOutputDataRate::Hz50).unwrap();
    
    let usart1 = unsafe { &mut *(USART1::ptr() as *mut _) };
    let mut serial = SerialPort::new(usart1);

    let maybe_sensor = sensor.into_mag_continuous();

    match maybe_sensor {
    Ok(mut sensor) => {
        loop {
            let data = sensor.mag_data()
                .expect("Reading not found.");


            // uprintln!(serial, "\rdata: {:?}", data);
            
            let direction = match (data.x > 0, data.y > 0) {
                (true, true) => Direction::Southeast,
                (false, true) => Direction::Northeast,
                (true, false) => Direction::Southwest,
                (false, false) => Direction::Northwest,
            };

            uprintln!(serial, "\rdirection: {:?}", direction);
    
            delay.delay_ms(200_u16);
            //     .expect("Cannot get magnetrometer measurements.");
    
            // uprintln!(serial, "\rMagnetrometer: x {} y {} z {}", data.x, data.y, data.z);
    
    
            //     let _data = sensor.accel_data().unwrap();
    
                // uprintln!("Acceleration: x {} y {} z {}", data.x, data.y, data.z);
            // }
        }
    },
    Err(_) => {
        panic!("error");
    },
    }
}
