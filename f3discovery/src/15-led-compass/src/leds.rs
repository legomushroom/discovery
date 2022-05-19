
use embedded_hal::digital::v2::PinState;
use hal::gpio::{Gpioe, U, Output, PushPull, Pin};
use hal::gpio::gpioe::Parts as GPIOE;
use stm32f3xx_hal::{self as hal, prelude::*};

#[allow(unused_imports)]
use m::Float;

use crate::CompassDirection;

pub struct LEDs {
    south: Pin<Gpioe, U<8>, Output<PushPull>>,
    south_west: Pin<Gpioe, U<9>, Output<PushPull>>,
    west: Pin<Gpioe, U<10>, Output<PushPull>>,
    north_west: Pin<Gpioe, U<11>, Output<PushPull>>,
    north: Pin<Gpioe, U<12>, Output<PushPull>>,
    north_east: Pin<Gpioe, U<13>, Output<PushPull>>,
    east: Pin<Gpioe, U<14>, Output<PushPull>>,
    south_east: Pin<Gpioe, U<15>, Output<PushPull>>,
}

impl LEDs {
    pub fn new(mut gpioe: GPIOE) -> LEDs {
        let south = gpioe.pe8.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let south_west = gpioe.pe9.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let west = gpioe.pe10.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let north_west = gpioe.pe11.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let north = gpioe.pe12.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let north_east = gpioe.pe13.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let east = gpioe.pe14.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let south_east = gpioe.pe15.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

        return LEDs {
            south,
            south_west,
            west,
            north_west,
            north,
            north_east,
            east,
            south_east,
        };
    }

    pub fn set_direction_led(
        &mut self,
        direction: CompassDirection,
    ) {
        // turn off all LEDs
        self.reset_all();

        // turn single LED according to `direction` value provided
        match direction {
            CompassDirection::South => {
                self.south.set_high().expect("cannot set `south` LED to high");
            },
            CompassDirection::SouthWest => {
                self.south_west.set_high().expect("cannot set `south_west` LED to high");
            },
            CompassDirection::West => {
                self.west.set_high().expect("cannot set `west` LED to high");
            },
            CompassDirection::NorthWest => {
                self.north_west.set_high().expect("cannot set `north_west` LED to high");
            },
            CompassDirection::North => {
                self.north.set_high().expect("cannot set `north` LED to high");
            },
            CompassDirection::NorthEast => {
                self.north_east.set_high().expect("cannot set `north_east` LED to high");
            },
            CompassDirection::East => {
                self.east.set_high().expect("cannot set `east` LED to high");
            },
            CompassDirection::SouthEast => {
                self.south_east.set_high().expect("cannot set `south_east` LED to high");
            },
        }
    }

    fn reset_all(&mut self) {
        self.south.set_state(PinState::Low).expect("cannot reset `south` LED");
        self.south_west.set_state(PinState::Low).expect("cannot reset `south_west` LED");
        self.west.set_state(PinState::Low).expect("cannot reset `west` LED");
        self.north_west.set_state(PinState::Low).expect("cannot reset `north_west` LED");
        self.north.set_state(PinState::Low).expect("cannot reset `north` LED");
        self.north_east.set_state(PinState::Low).expect("cannot reset `north_east` LED");
        self.east.set_state(PinState::Low).expect("cannot reset `east` LED");
        self.south_east.set_state(PinState::Low).expect("cannot reset `south_east` LED");
    }
}
