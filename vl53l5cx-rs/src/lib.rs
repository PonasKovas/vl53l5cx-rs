//! Safe `no_std` driver for the VL53L5CX, built on ST's C ULD (blocking).
//!
//! Works with any `embedded_hal::i2c::I2c` (1.0), including `embassy-stm32`'s
//! `I2c` in both `Blocking` and `Async` mode (the blocking trait is used).
//! Delays use `embedded_hal::delay::DelayNs` (e.g. `embassy_time::Delay`).
//!
//! Power/reset pins (LPn, I2C_RST) are left to the caller: have the sensor
//! powered with LPn high before calling [`Vl53l5cx::init`].
//!
//! ## Feature Flags
//!
//! ### Measurement Outputs
//! Disabling unused outputs reduces I2C transfer sizes and sensor memory footprint.
//! - `distance-mm` *(default)*: Target distance in millimeters.
//! - `target-status` *(default)*: Measurement validity status per target (5 and 9 indicate valid range).
//! - `signal-per-spad` *(default)*: Returned signal rate in kcps/SPAD.
//! - `ambient-per-spad` *(default)*: Ambient noise rate in kcps/SPAD.
//! - `nb-target-detected` *(default)*: Number of detected targets in each zone.
//! - `reflectance-percent` *(default)*: Estimated target reflectance percentage.
//! - `range-sigma-mm`: Standard deviation (sigma) of distance measurement in millimeters.
//! - `nb-spads-enabled`: Number of active SPADs for each zone.
//!
//! ### Targets per Zone
//! Maximum targets tracked per zone (default is 1). Choose at most one:
//! - `targets-2`: Up to 2 targets per zone.
//! - `targets-3`: Up to 3 targets per zone.
//! - `targets-4`: Up to 4 targets per zone.
//!
//! ### ST Plugins
//! Optional firmware plugins compiled into the driver:
//! - `xtalk`: Crosstalk calibration routines and margin configuration.
//! - `detection-thresholds`: Programmable per-zone detection thresholds and interrupts.
//! - `motion-indicator`: Motion detection indicator across zones.
//! - `plugins`: Convenience flag enabling `xtalk`, `detection-thresholds`, and `motion-indicator`.
//!
//! ### Other
//! - `defmt`: Derives `defmt::Format` on public data types and errors.
#![no_std]
#![feature(doc_cfg)]

mod driver;
mod error;
mod platform;
mod results;
#[cfg(feature = "detection-thresholds")]
mod thresholds;
mod types;

pub use driver::{Ranging, Ready, Uninit, Vl53l5cx};
pub use error::Error;
pub use results::Results;
#[cfg(feature = "motion-indicator")]
#[doc(cfg(feature = "motion-indicator"))]
pub use results::MotionIndicator;
#[cfg(feature = "detection-thresholds")]
#[doc(cfg(feature = "detection-thresholds"))]
pub use thresholds::{
    DetectionThreshold, DetectionThresholds, Measurement, Operation, WindowType,
    MAX_THRESHOLDS,
};
pub use types::{PowerMode, RangingMode, Resolution, TargetOrder};

/// Default 7-bit I2C address (ST's 8-bit 0x52).
pub const DEFAULT_I2C_ADDRESS: u8 = 0x29;

/// Targets reported per zone (compile-time, set by the `targets-N` features).
pub const TARGETS_PER_ZONE: usize = if cfg!(feature = "targets-4") {
    4
} else if cfg!(feature = "targets-3") {
    3
} else if cfg!(feature = "targets-2") {
    2
} else {
    1
};

/// Size of the crosstalk calibration blob.
#[cfg(feature = "xtalk")]
#[doc(cfg(feature = "xtalk"))]
pub const XTALK_DATA_SIZE: usize = vl53l5cx_sys::VL53L5CX_XTALK_BUFFER_SIZE;
