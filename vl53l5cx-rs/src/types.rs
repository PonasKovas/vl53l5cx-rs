use vl53l5cx_sys as sys;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resolution {
    R4x4,
    R8x8,
}

impl Resolution {
    /// Number of zones.
    pub const fn zones(self) -> usize {
        match self {
            Self::R4x4 => 16,
            Self::R8x8 => 64,
        }
    }
    pub(crate) fn raw(self) -> u8 {
        match self {
            Self::R4x4 => sys::VL53L5CX_RESOLUTION_4X4,
            Self::R8x8 => sys::VL53L5CX_RESOLUTION_8X8,
        }
    }
    pub(crate) fn from_raw(v: u8) -> Option<Self> {
        match v {
            sys::VL53L5CX_RESOLUTION_4X4 => Some(Self::R4x4),
            sys::VL53L5CX_RESOLUTION_8X8 => Some(Self::R8x8),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TargetOrder {
    Closest,
    Strongest,
}

impl TargetOrder {
    pub(crate) fn raw(self) -> u8 {
        match self {
            Self::Closest => sys::VL53L5CX_TARGET_ORDER_CLOSEST,
            Self::Strongest => sys::VL53L5CX_TARGET_ORDER_STRONGEST,
        }
    }
    pub(crate) fn from_raw(v: u8) -> Option<Self> {
        match v {
            sys::VL53L5CX_TARGET_ORDER_CLOSEST => Some(Self::Closest),
            sys::VL53L5CX_TARGET_ORDER_STRONGEST => Some(Self::Strongest),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RangingMode {
    Continuous,
    /// Integration time is only settable in this mode.
    Autonomous,
}

impl RangingMode {
    pub(crate) fn raw(self) -> u8 {
        match self {
            Self::Continuous => sys::VL53L5CX_RANGING_MODE_CONTINUOUS,
            Self::Autonomous => sys::VL53L5CX_RANGING_MODE_AUTONOMOUS,
        }
    }
    pub(crate) fn from_raw(v: u8) -> Option<Self> {
        match v {
            sys::VL53L5CX_RANGING_MODE_CONTINUOUS => Some(Self::Continuous),
            sys::VL53L5CX_RANGING_MODE_AUTONOMOUS => Some(Self::Autonomous),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PowerMode {
    Sleep,
    Wakeup,
}

impl PowerMode {
    pub(crate) fn raw(self) -> u8 {
        match self {
            Self::Sleep => sys::VL53L5CX_POWER_MODE_SLEEP,
            Self::Wakeup => sys::VL53L5CX_POWER_MODE_WAKEUP,
        }
    }
    pub(crate) fn from_raw(v: u8) -> Option<Self> {
        match v {
            sys::VL53L5CX_POWER_MODE_SLEEP => Some(Self::Sleep),
            sys::VL53L5CX_POWER_MODE_WAKEUP => Some(Self::Wakeup),
            _ => None,
        }
    }
}
