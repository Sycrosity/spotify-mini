#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(error_in_core)]
#![allow(clippy::unused_unit, clippy::const_is_empty)]

// #[cfg(feature = "adc")]
// pub mod adc;
#[cfg(feature = "alloc")]
pub mod alloc;
pub mod blink;
pub mod display;
pub mod errors;

pub mod logger;
#[cfg(feature = "net")]
pub mod net;
pub mod volume;

pub mod prelude {

    pub const SSID: &str = env!("SSID");
    pub const PASSWORD: &str = env!("PASSWORD");

    pub const TICKS_PER_SECOND: u64 = 16_000_000;

    pub use core::f64::consts::PI;

    pub use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
    pub use embassy_sync::{
        blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex},
        mutex::Mutex,
        pubsub::PubSubChannel,
        signal::Signal,
    };

    pub use crate::errors::*;

    pub type SharedI2C =
        I2cDevice<'static, NoopRawMutex, I2C<'static, esp_hal::peripherals::I2C0, Async>>;

    pub static VOLUME_CHANNEL: PubSubChannel<CriticalSectionRawMutex, f32, 4, 2, 1> =
        PubSubChannel::new();

    pub static I2C_BUS: StaticCell<I2cBusMutex> = StaticCell::new();

    pub type I2cBusMutex = Mutex<NoopRawMutex, I2C<'static, esp_hal::peripherals::I2C0, Async>>;

    pub static SHARED_ADC: StaticCell<ADCMutex> = StaticCell::new();

    pub type ADCMutex = Mutex<CriticalSectionRawMutex, ADC<'static, esp_hal::peripherals::ADC1>>;

    pub static RNG: StaticCell<Rng> = StaticCell::new();

    pub use embassy_executor::task;
    pub use embassy_time::{Delay, Duration, Instant, Ticker, Timer};
    #[allow(unused)]
    pub use esp_backtrace as _;
    pub use esp_hal::{
        analog::adc::ADC,
        embassy,
        gpio::{AnyPin, Output, PushPull},
        i2c::I2C,
        prelude::*,
        rng::Rng,
        Async,
    };
    pub use esp_println::{print, println};
    pub use heapless::String;
    pub use log::{debug, error, info, log, trace, warn};
    pub use nb::block;
    pub use ssd1306::prelude::*;
    pub use static_cell::make_static;
    use static_cell::StaticCell;
}
