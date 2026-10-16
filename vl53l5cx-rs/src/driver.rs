use core::{ffi::c_void, marker::PhantomData, mem, ptr};
use embedded_hal::{delay::DelayNs, i2c::I2c};
use vl53l5cx_sys as sys;

use crate::{
    platform::{self, Bus},
    Error, PowerMode, RangingMode, Resolution, Results, TargetOrder, DEFAULT_I2C_ADDRESS,
};

/// State: created, firmware not loaded.
pub struct Uninit;
/// State: initialised, configurable, not ranging.
pub struct Ready;
/// State: ranging.
pub struct Ranging;

/// VL53L5CX driver. State transitions consume the driver; on failure the
/// previous-state driver is handed back together with the error.
pub struct Vl53l5cx<I2C: I2c, D: DelayNs, S> {
    bus: Bus<I2C, D>,
    dev: sys::VL53L5CX_Configuration,
    results: sys::VL53L5CX_ResultsData,
    zones: u8,
    _state: PhantomData<S>,
}

// ---------------------------------------------------------------- any state
impl<I2C: I2c, D: DelayNs, S> Vl53l5cx<I2C, D, S> {
    fn cast<N>(self) -> Vl53l5cx<I2C, D, N> {
        Vl53l5cx {
            bus: self.bus,
            dev: self.dev,
            results: self.results,
            zones: self.zones,
            _state: PhantomData,
        }
    }

    fn begin(&mut self) {
        self.bus.error = None;
        self.dev.platform.ctx = (&mut self.bus as *mut Bus<I2C, D>).cast::<c_void>();
    }

    fn finish(&mut self, status: u8) -> Result<(), Error<I2C::Error>> {
        self.dev.platform.ctx = ptr::null_mut();
        if let Some(e) = self.bus.error.take() {
            return Err(Error::I2c(e));
        }
        match status {
            sys::VL53L5CX_STATUS_OK => Ok(()),
            sys::VL53L5CX_STATUS_TIMEOUT_ERROR => Err(Error::Timeout),
            sys::VL53L5CX_STATUS_CORRUPTED_FRAME => Err(Error::CorruptedFrame),
            sys::VL53L5CX_STATUS_CRC_CSUM_FAILED => Err(Error::CrcChecksum),
            sys::VL53L5CX_STATUS_INVALID_PARAM => Err(Error::InvalidParam),
            s => Err(Error::Uld(s)),
        }
    }

    fn call(
        &mut self,
        f: impl FnOnce(*mut sys::VL53L5CX_Configuration) -> u8,
    ) -> Result<(), Error<I2C::Error>> {
        self.begin();
        let status = f(&mut self.dev);
        self.finish(status)
    }

    /// Change the sensor's I2C address (7-bit). Other sensors on the bus must
    /// have LPn low while doing this.
    pub fn set_i2c_address(&mut self, address: u8) -> Result<(), Error<I2C::Error>> {
        if address > 0x7f {
            return Err(Error::InvalidParam);
        }
        self.call(|dev| unsafe { sys::vl53l5cx_set_i2c_address(dev, u16::from(address) << 1) })
    }

    /// Give back the bus and delay.
    pub fn release(self) -> (I2C, D) {
        (self.bus.i2c, self.bus.delay)
    }
}

// ------------------------------------------------------------------- Uninit
impl<I2C: I2c, D: DelayNs> Vl53l5cx<I2C, D, Uninit> {
    /// New driver at the default address (0x29).
    pub fn new(i2c: I2C, delay: D) -> Self {
        Self::with_address(i2c, delay, DEFAULT_I2C_ADDRESS)
    }

    /// New driver at a 7-bit address.
    pub fn with_address(i2c: I2C, delay: D, address: u8) -> Self {
        let mut dev: sys::VL53L5CX_Configuration = unsafe { mem::zeroed() };
        dev.platform.address = u16::from(address) << 1;
        dev.platform.read = Some(platform::read_cb::<I2C, D>);
        dev.platform.write = Some(platform::write_cb::<I2C, D>);
        dev.platform.wait_ms = Some(platform::wait_cb::<I2C, D>);
        Self {
            bus: Bus { i2c, delay, error: None },
            dev,
            results: unsafe { mem::zeroed() },
            zones: 0,
            _state: PhantomData,
        }
    }

    /// Check that the sensor answers on the bus.
    pub fn is_alive(&mut self) -> Result<bool, Error<I2C::Error>> {
        let mut alive = 0u8;
        self.call(|dev| unsafe { sys::vl53l5cx_is_alive(dev, &mut alive) })?;
        Ok(alive != 0)
    }

    /// Upload the firmware and load defaults. Takes a second or two.
    pub fn init(mut self) -> Result<Vl53l5cx<I2C, D, Ready>, (Self, Error<I2C::Error>)> {
        match self.call(|dev| unsafe { sys::vl53l5cx_init(dev) }) {
            Ok(()) => Ok(self.cast()),
            Err(e) => Err((self, e)),
        }
    }
}

// -------------------------------------------------------------------- Ready
impl<I2C: I2c, D: DelayNs> Vl53l5cx<I2C, D, Ready> {
    pub fn resolution(&mut self) -> Result<Resolution, Error<I2C::Error>> {
        let mut v = 0u8;
        self.call(|dev| unsafe { sys::vl53l5cx_get_resolution(dev, &mut v) })?;
        Resolution::from_raw(v).ok_or(Error::Uld(v))
    }

    /// Set before the ranging frequency: its valid range depends on this.
    pub fn set_resolution(&mut self, resolution: Resolution) -> Result<(), Error<I2C::Error>> {
        self.call(|dev| unsafe { sys::vl53l5cx_set_resolution(dev, resolution.raw()) })
    }

    pub fn ranging_frequency_hz(&mut self) -> Result<u8, Error<I2C::Error>> {
        let mut v = 0u8;
        self.call(|dev| unsafe { sys::vl53l5cx_get_ranging_frequency_hz(dev, &mut v) })?;
        Ok(v)
    }

    /// 1..=60 Hz for 4x4, 1..=15 Hz for 8x8.
    pub fn set_ranging_frequency_hz(&mut self, hz: u8) -> Result<(), Error<I2C::Error>> {
        self.call(|dev| unsafe { sys::vl53l5cx_set_ranging_frequency_hz(dev, hz) })
    }

    pub fn integration_time_ms(&mut self) -> Result<u32, Error<I2C::Error>> {
        let mut v = 0u32;
        self.call(|dev| unsafe { sys::vl53l5cx_get_integration_time_ms(dev, &mut v) })?;
        Ok(v)
    }

    /// Only valid in [`RangingMode::Autonomous`].
    pub fn set_integration_time_ms(&mut self, ms: u32) -> Result<(), Error<I2C::Error>> {
        self.call(|dev| unsafe { sys::vl53l5cx_set_integration_time_ms(dev, ms) })
    }

    pub fn sharpener_percent(&mut self) -> Result<u8, Error<I2C::Error>> {
        let mut v = 0u8;
        self.call(|dev| unsafe { sys::vl53l5cx_get_sharpener_percent(dev, &mut v) })?;
        Ok(v)
    }

    /// 0..=99.
    pub fn set_sharpener_percent(&mut self, percent: u8) -> Result<(), Error<I2C::Error>> {
        self.call(|dev| unsafe { sys::vl53l5cx_set_sharpener_percent(dev, percent) })
    }

    pub fn target_order(&mut self) -> Result<TargetOrder, Error<I2C::Error>> {
        let mut v = 0u8;
        self.call(|dev| unsafe { sys::vl53l5cx_get_target_order(dev, &mut v) })?;
        TargetOrder::from_raw(v).ok_or(Error::Uld(v))
    }

    pub fn set_target_order(&mut self, order: TargetOrder) -> Result<(), Error<I2C::Error>> {
        self.call(|dev| unsafe { sys::vl53l5cx_set_target_order(dev, order.raw()) })
    }

    pub fn ranging_mode(&mut self) -> Result<RangingMode, Error<I2C::Error>> {
        let mut v = 0u8;
        self.call(|dev| unsafe { sys::vl53l5cx_get_ranging_mode(dev, &mut v) })?;
        RangingMode::from_raw(v).ok_or(Error::Uld(v))
    }

    pub fn set_ranging_mode(&mut self, mode: RangingMode) -> Result<(), Error<I2C::Error>> {
        self.call(|dev| unsafe { sys::vl53l5cx_set_ranging_mode(dev, mode.raw()) })
    }

    pub fn power_mode(&mut self) -> Result<PowerMode, Error<I2C::Error>> {
        let mut v = 0u8;
        self.call(|dev| unsafe { sys::vl53l5cx_get_power_mode(dev, &mut v) })?;
        PowerMode::from_raw(v).ok_or(Error::Uld(v))
    }

    /// The sensor must be in `Wakeup` to range.
    pub fn set_power_mode(&mut self, mode: PowerMode) -> Result<(), Error<I2C::Error>> {
        self.call(|dev| unsafe { sys::vl53l5cx_set_power_mode(dev, mode.raw()) })
    }

    pub fn start_ranging(
        mut self,
    ) -> Result<Vl53l5cx<I2C, D, Ranging>, (Self, Error<I2C::Error>)> {
        let res = match self.resolution() {
            Ok(r) => r,
            Err(e) => return Err((self, e)),
        };
        match self.call(|dev| unsafe { sys::vl53l5cx_start_ranging(dev) }) {
            Ok(()) => {
                self.zones = res.zones() as u8;
                Ok(self.cast())
            }
            Err(e) => Err((self, e)),
        }
    }
}

// ------------------------------------------------------------------ Ranging
impl<I2C: I2c, D: DelayNs> Vl53l5cx<I2C, D, Ranging> {
    /// Poll whether a new frame is available.
    pub fn check_data_ready(&mut self) -> Result<bool, Error<I2C::Error>> {
        let mut ready = 0u8;
        self.call(|dev| unsafe { sys::vl53l5cx_check_data_ready(dev, &mut ready) })?;
        Ok(ready != 0)
    }

    /// Read the latest frame. Call after `check_data_ready` returned true.
    pub fn get_ranging_data(&mut self) -> Result<Results<'_>, Error<I2C::Error>> {
        self.begin();
        let status =
            unsafe { sys::vl53l5cx_get_ranging_data(&mut self.dev, &mut self.results) };
        self.finish(status)?;
        Ok(Results {
            data: &self.results,
            zones: usize::from(self.zones),
        })
    }

    pub fn stop_ranging(
        mut self,
    ) -> Result<Vl53l5cx<I2C, D, Ready>, (Self, Error<I2C::Error>)> {
        match self.call(|dev| unsafe { sys::vl53l5cx_stop_ranging(dev) }) {
            Ok(()) => Ok(self.cast()),
            Err(e) => Err((self, e)),
        }
    }
}
