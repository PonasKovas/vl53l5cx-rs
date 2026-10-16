//! Raw bindings to ST's VL53L5CX ULD. Use `vl53l5cx-rs` for the safe API.
#![no_std]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Constants from vl53l5cx_api.h (cast-style macros, so written by hand).
pub const VL53L5CX_DEFAULT_I2C_ADDRESS: u16 = 0x52; // 8-bit form

pub const VL53L5CX_RESOLUTION_4X4: u8 = 16;
pub const VL53L5CX_RESOLUTION_8X8: u8 = 64;

pub const VL53L5CX_TARGET_ORDER_CLOSEST: u8 = 1;
pub const VL53L5CX_TARGET_ORDER_STRONGEST: u8 = 2;

pub const VL53L5CX_RANGING_MODE_CONTINUOUS: u8 = 1;
pub const VL53L5CX_RANGING_MODE_AUTONOMOUS: u8 = 3;

pub const VL53L5CX_POWER_MODE_SLEEP: u8 = 0;
pub const VL53L5CX_POWER_MODE_WAKEUP: u8 = 1;

pub const VL53L5CX_STATUS_OK: u8 = 0;
pub const VL53L5CX_STATUS_TIMEOUT_ERROR: u8 = 1;
pub const VL53L5CX_STATUS_CORRUPTED_FRAME: u8 = 2;
pub const VL53L5CX_STATUS_CRC_CSUM_FAILED: u8 = 3;
pub const VL53L5CX_STATUS_INVALID_PARAM: u8 = 127;
pub const VL53L5CX_STATUS_ERROR: u8 = 255;
