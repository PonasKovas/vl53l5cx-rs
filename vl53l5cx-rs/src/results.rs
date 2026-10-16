use crate::TARGETS_PER_ZONE;
use vl53l5cx_sys as sys;

/// Borrowed view of the last ranging frame. Per-target arrays are indexed
/// `zone * TARGETS_PER_ZONE + target`; zones are row-major (4x4 or 8x8).
/// Accessors return `None` when `zone`/`target` is out of range.
pub struct Results<'a> {
    pub(crate) data: &'a sys::VL53L5CX_ResultsData,
    pub(crate) zones: usize,
}

#[allow(dead_code)]
impl<'a> Results<'a> {
    /// Number of zones in this frame (16 or 64).
    pub fn zones(&self) -> usize {
        self.zones
    }

    /// Silicon temperature in °C.
    pub fn silicon_temp_degc(&self) -> i8 {
        self.data.silicon_temp_degc
    }

    fn zt(&self, zone: usize, target: usize) -> Option<usize> {
        (zone < self.zones && target < TARGETS_PER_ZONE).then(|| zone * TARGETS_PER_ZONE + target)
    }

    fn z(&self, zone: usize) -> Option<usize> {
        (zone < self.zones).then_some(zone)
    }

    /// Distance in mm.
    #[cfg(feature = "distance-mm")]
    pub fn distance_mm(&self, zone: usize, target: usize) -> Option<i16> {
        self.zt(zone, target).map(|i| self.data.distance_mm[i])
    }

    /// Validity of the measurement; 5 and 9 mean the range is OK.
    #[cfg(feature = "target-status")]
    pub fn target_status(&self, zone: usize, target: usize) -> Option<u8> {
        self.zt(zone, target).map(|i| self.data.target_status[i])
    }

    /// Returned signal in kcps/spad.
    #[cfg(feature = "signal-per-spad")]
    pub fn signal_per_spad(&self, zone: usize, target: usize) -> Option<u32> {
        self.zt(zone, target).map(|i| self.data.signal_per_spad[i])
    }

    /// Estimated reflectance in percent.
    #[cfg(feature = "reflectance-percent")]
    pub fn reflectance_percent(&self, zone: usize, target: usize) -> Option<u8> {
        self.zt(zone, target).map(|i| self.data.reflectance[i])
    }

    /// Sigma of the distance in mm.
    #[cfg(feature = "range-sigma-mm")]
    pub fn range_sigma_mm(&self, zone: usize, target: usize) -> Option<u16> {
        self.zt(zone, target).map(|i| self.data.range_sigma_mm[i])
    }

    /// Ambient noise in kcps/spad.
    #[cfg(feature = "ambient-per-spad")]
    pub fn ambient_per_spad(&self, zone: usize) -> Option<u32> {
        self.z(zone).map(|i| self.data.ambient_per_spad[i])
    }

    /// Number of valid targets detected in the zone.
    #[cfg(feature = "nb-target-detected")]
    pub fn nb_target_detected(&self, zone: usize) -> Option<u8> {
        self.z(zone).map(|i| self.data.nb_target_detected[i])
    }

    /// Number of SPADs enabled for the zone.
    #[cfg(feature = "nb-spads-enabled")]
    pub fn nb_spads_enabled(&self, zone: usize) -> Option<u32> {
        self.z(zone).map(|i| self.data.nb_spads_enabled[i])
    }
}
