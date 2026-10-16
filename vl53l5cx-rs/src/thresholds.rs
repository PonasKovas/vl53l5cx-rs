use vl53l5cx_sys as sys;

/// Maximum number of checkers the sensor holds.
pub const MAX_THRESHOLDS: usize = sys::VL53L5CX_NB_THRESHOLDS;

/// What a checker measures. Units of `low`/`high` follow the measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Measurement {
    DistanceMm,
    SignalPerSpadKcps,
    RangeSigmaMm,
    AmbientPerSpadKcps,
    NbTargetDetected,
    TargetStatus,
    NbSpadsEnabled,
    MotionIndicator,
}

impl Measurement {
    fn raw(self) -> u8 {
        match self {
            Self::DistanceMm => sys::VL53L5CX_DISTANCE_MM,
            Self::SignalPerSpadKcps => sys::VL53L5CX_SIGNAL_PER_SPAD_KCPS,
            Self::RangeSigmaMm => sys::VL53L5CX_RANGE_SIGMA_MM,
            Self::AmbientPerSpadKcps => sys::VL53L5CX_AMBIENT_PER_SPAD_KCPS,
            Self::NbTargetDetected => sys::VL53L5CX_NB_TARGET_DETECTED,
            Self::TargetStatus => sys::VL53L5CX_TARGET_STATUS,
            Self::NbSpadsEnabled => sys::VL53L5CX_NB_SPADS_ENABLED,
            Self::MotionIndicator => sys::VL53L5CX_MOTION_INDICATOR,
        }
    }
    fn from_raw(v: u8) -> Option<Self> {
        match v {
            sys::VL53L5CX_DISTANCE_MM => Some(Self::DistanceMm),
            sys::VL53L5CX_SIGNAL_PER_SPAD_KCPS => Some(Self::SignalPerSpadKcps),
            sys::VL53L5CX_RANGE_SIGMA_MM => Some(Self::RangeSigmaMm),
            sys::VL53L5CX_AMBIENT_PER_SPAD_KCPS => Some(Self::AmbientPerSpadKcps),
            sys::VL53L5CX_NB_TARGET_DETECTED => Some(Self::NbTargetDetected),
            sys::VL53L5CX_TARGET_STATUS => Some(Self::TargetStatus),
            sys::VL53L5CX_NB_SPADS_ENABLED => Some(Self::NbSpadsEnabled),
            sys::VL53L5CX_MOTION_INDICATOR => Some(Self::MotionIndicator),
            _ => None,
        }
    }
}

/// How `low`/`high` are compared with the measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WindowType {
    InWindow,
    OutOfWindow,
    LessThanEqualMin,
    GreaterThanMax,
    EqualMin,
    NotEqualMin,
}

impl WindowType {
    fn raw(self) -> u8 {
        match self {
            Self::InWindow => sys::VL53L5CX_IN_WINDOW,
            Self::OutOfWindow => sys::VL53L5CX_OUT_OF_WINDOW,
            Self::LessThanEqualMin => sys::VL53L5CX_LESS_THAN_EQUAL_MIN_CHECKER,
            Self::GreaterThanMax => sys::VL53L5CX_GREATER_THAN_MAX_CHECKER,
            Self::EqualMin => sys::VL53L5CX_EQUAL_MIN_CHECKER,
            Self::NotEqualMin => sys::VL53L5CX_NOT_EQUAL_MIN_CHECKER,
        }
    }
    fn from_raw(v: u8) -> Option<Self> {
        match v {
            sys::VL53L5CX_IN_WINDOW => Some(Self::InWindow),
            sys::VL53L5CX_OUT_OF_WINDOW => Some(Self::OutOfWindow),
            sys::VL53L5CX_LESS_THAN_EQUAL_MIN_CHECKER => Some(Self::LessThanEqualMin),
            sys::VL53L5CX_GREATER_THAN_MAX_CHECKER => Some(Self::GreaterThanMax),
            sys::VL53L5CX_EQUAL_MIN_CHECKER => Some(Self::EqualMin),
            sys::VL53L5CX_NOT_EQUAL_MIN_CHECKER => Some(Self::NotEqualMin),
            _ => None,
        }
    }
}

/// How a checker combines with the previous one in the same zone. The first
/// checker must be `Or`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Operation {
    Or,
    And,
}

impl Operation {
    fn raw(self) -> u8 {
        match self {
            Self::Or => sys::VL53L5CX_OPERATION_OR,
            Self::And => sys::VL53L5CX_OPERATION_AND,
        }
    }
    fn from_raw(v: u8) -> Option<Self> {
        match v {
            sys::VL53L5CX_OPERATION_OR => Some(Self::Or),
            sys::VL53L5CX_OPERATION_AND => Some(Self::And),
            _ => None,
        }
    }
}

/// One detection checker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DetectionThreshold {
    pub measurement: Measurement,
    pub window: WindowType,
    /// Zone index (see ST's user manual for the zone numbering).
    pub zone: u8,
    pub operation: Operation,
    pub low: i32,
    pub high: i32,
}

impl DetectionThreshold {
    pub(crate) fn to_raw(&self) -> sys::VL53L5CX_DetectionThresholds {
        sys::VL53L5CX_DetectionThresholds {
            param_low_thresh: self.low,
            param_high_thresh: self.high,
            measurement: self.measurement.raw(),
            type_: self.window.raw(),
            zone_num: self.zone,
            mathematic_operation: self.operation.raw(),
        }
    }
}

const FILLER: DetectionThreshold = DetectionThreshold {
    measurement: Measurement::DistanceMm,
    window: WindowType::InWindow,
    zone: 0,
    operation: Operation::Or,
    low: 0,
    high: 0,
};

/// Checkers read back from the sensor.
pub struct DetectionThresholds {
    items: [DetectionThreshold; MAX_THRESHOLDS],
    len: usize,
}

impl DetectionThresholds {
    pub fn as_slice(&self) -> &[DetectionThreshold] {
        &self.items[..self.len]
    }

    /// Err carries the first raw measurement/type/operation byte that
    /// could not be decoded.
    pub(crate) fn from_raw(
        raw: &[sys::VL53L5CX_DetectionThresholds; MAX_THRESHOLDS],
    ) -> Result<Self, u8> {
        let len = raw
            .iter()
            .position(|r| r.zone_num == sys::VL53L5CX_LAST_THRESHOLD)
            .unwrap_or(MAX_THRESHOLDS);
        let mut items = [FILLER; MAX_THRESHOLDS];
        for (item, r) in items.iter_mut().zip(&raw[..len]) {
            *item = DetectionThreshold {
                measurement: Measurement::from_raw(r.measurement).ok_or(r.measurement)?,
                window: WindowType::from_raw(r.type_).ok_or(r.type_)?,
                zone: r.zone_num,
                operation: Operation::from_raw(r.mathematic_operation)
                    .ok_or(r.mathematic_operation)?,
                low: r.param_low_thresh,
                high: r.param_high_thresh,
            };
        }
        Ok(Self { items, len })
    }
}
