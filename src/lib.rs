#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(error_in_core)]
#![allow(clippy::unused_unit)]

// #[cfg(feature = "adc")]
// pub mod adc;
#[cfg(feature = "alloc")]
pub mod alloc;
pub mod blink;
pub mod display;
pub mod errors;

pub mod buttons;
pub mod logger;
#[cfg(feature = "net")]
pub mod net;
pub mod potentiometer;
pub mod volume;

pub mod prelude {

    pub const SSID: &str = env!("SSID");
    pub const PASSWORD: &str = env!("PASSWORD");

    pub use core::f64::consts::PI;

    pub use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
    pub use embassy_sync::{
        blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex},
        mutex::Mutex,
        pubsub::PubSubChannel,
        signal::Signal,
    };

    pub use crate::errors::*;

    pub type SharedI2C = I2cDevice<
        'static,
        embassy_sync::blocking_mutex::raw::NoopRawMutex,
        I2C<'static, esp_hal::peripherals::I2C0, Async>,
    >;

    pub static VOLUME_CHANNEL: PubSubChannel<CriticalSectionRawMutex, f32, 2, 2, 1> =
        PubSubChannel::new();

    pub static I2C_BUS: StaticCell<I2cBusMutex> = StaticCell::new();

    pub type I2cBusMutex = Mutex<NoopRawMutex, I2C<'static, esp_hal::peripherals::I2C0, Async>>;

    pub static SHARED_ADC: StaticCell<AdcMutex> = StaticCell::new();

    pub type AdcMutex = Mutex<CriticalSectionRawMutex, Adc<'static, esp_hal::peripherals::ADC1>>;

    pub static RNG: StaticCell<Rng> = StaticCell::new();

    pub use embassy_executor::task;
    pub use embassy_time::{Delay, Duration, Instant, Ticker, Timer};
    #[allow(unused)]
    pub use esp_backtrace as _;
    pub use esp_hal::{
        analog::adc::Adc,
        gpio::{Analog, AnyInput, AnyOutput, Input, Output},
        i2c::I2C,
        prelude::*,
        rng::Rng,
        Async,
    };
    pub use esp_println::{print, println};
    pub use heapless::{String, Vec};
    pub use log::{debug, error, info, log, trace, warn};
    pub use nb::block;
    pub use ssd1306::{prelude::*, Ssd1306};
    pub use static_cell::{make_static, StaticCell};
}
