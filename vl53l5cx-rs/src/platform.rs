//! Callbacks the C platform layer uses to reach the Rust I2C bus and delay.
use core::ffi::c_void;
use embedded_hal::{
    delay::DelayNs,
    i2c::{I2c, Operation},
};
use vl53l5cx_sys as sys;

/// Everything the C callbacks need. Reached through `Platform::ctx`, which is
/// set to point at this on every FFI call.
pub(crate) struct Bus<I2C: I2c, D: DelayNs> {
    pub i2c: I2C,
    pub delay: D,
    /// First bus error seen during the current FFI call.
    pub error: Option<I2C::Error>,
}

pub(crate) unsafe extern "C" fn read_cb<I2C: I2c, D: DelayNs>(
    ctx: *mut c_void,
    addr: u8,
    reg: u16,
    buf: *mut u8,
    len: u32,
) -> u8 {
    let bus = &mut *(ctx as *mut Bus<I2C, D>);
    if bus.error.is_some() {
        return sys::VL53L5CX_STATUS_ERROR;
    }
    let buf = core::slice::from_raw_parts_mut(buf, len as usize);
    match bus.i2c.write_read(addr, &reg.to_be_bytes(), buf) {
        Ok(()) => 0,
        Err(e) => {
            bus.error = Some(e);
            sys::VL53L5CX_STATUS_ERROR
        }
    }
}

pub(crate) unsafe extern "C" fn write_cb<I2C: I2c, D: DelayNs>(
    ctx: *mut c_void,
    addr: u8,
    reg: u16,
    buf: *const u8,
    len: u32,
) -> u8 {
    let bus = &mut *(ctx as *mut Bus<I2C, D>);
    if bus.error.is_some() {
        return sys::VL53L5CX_STATUS_ERROR;
    }
    let buf = core::slice::from_raw_parts(buf, len as usize);
    // Two consecutive writes are sent as one transfer (no copy, no restart).
    match bus
        .i2c
        .transaction(addr, &mut [Operation::Write(&reg.to_be_bytes()), Operation::Write(buf)])
    {
        Ok(()) => 0,
        Err(e) => {
            bus.error = Some(e);
            sys::VL53L5CX_STATUS_ERROR
        }
    }
}

pub(crate) unsafe extern "C" fn wait_cb<I2C: I2c, D: DelayNs>(ctx: *mut c_void, ms: u32) -> u8 {
    let bus = &mut *(ctx as *mut Bus<I2C, D>);
    // After a bus error the ULD's poll loops spin to their timeout; don't wait.
    if bus.error.is_none() {
        bus.delay.delay_ms(ms);
    }
    0
}
