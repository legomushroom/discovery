//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] //  bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

use core::fmt;
use core::fmt::Write;

pub use cortex_m::{asm::bkpt, iprint, iprintln};
pub use cortex_m_rt::entry;
use lsm303agr::{AccelOutputDataRate, Lsm303agr,};
pub use stm32f3_discovery::stm32f3xx_hal::{delay::Delay, prelude::*};

use stm32f3_discovery::{stm32f3xx_hal::{i2c::I2c, pac::{I2C1, Peripherals, USART1, i2c1, usart1}, serial::Serial}};

use embedded_time::rate::{Extensions, Hertz};

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


pub fn init() -> (&'static i2c1::RegisterBlock, Delay) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

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
    // If you are having trouble sending/receiving data to/from the
    // HC-05 bluetooth module, try this configuration instead:
    // Serial::usart1(dp.USART1, (tx, rx), 9600.bps(), clocks, &mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob.pb6.into_af4_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);

    let i2c = I2c::new(dp.I2C1, (scl, sda), Hertz::new(400_000), clocks, &mut rcc.apb1);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();

    let usart1 = unsafe { &mut *(USART1::ptr() as *mut _) };

    let delay = Delay::new(cp.SYST, clocks);
    let mut serial = SerialPort::new(usart1);

    loop {
        if sensor.accel_status().unwrap().xyz_new_data {
            let data = sensor.accel_data().unwrap();
            // bkpt();
            uprintln!(serial, "\rdata: x {} y {} z {}", data.x, data.y, data.z);
        }

        // delay.delay_ms(1000_u16);

        if 2 + 2 == 5 {
            break;
        }
    }

    unsafe { (&mut *(I2C1::ptr() as *mut _), delay) }
}
