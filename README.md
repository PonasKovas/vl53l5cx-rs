# vl53l5cx-rs

Safe, `no_std` Rust driver for the STMicroelectronics VL53L5CX 8x8 multizone Time-of-Flight ranging sensor, built on ST's Ultra Lite Driver (ULD).

## Feature Flags

Feature flags configure which data fields the sensor streams over I2C and which optional firmware plugins are included. Disabling unused outputs reduces I2C payload size and sensor RAM usage.

### Measurement Outputs
- **`distance-mm`** *(default)*: Target distance in millimeters.
- **`target-status`** *(default)*: Target validity status byte (values 5 and 9 indicate a valid range).
- **`signal-per-spad`** *(default)*: Return signal rate in kcps/SPAD.
- **`ambient-per-spad`** *(default)*: Ambient noise rate in kcps/SPAD.
- **`nb-target-detected`** *(default)*: Count of valid targets detected per zone.
- **`reflectance-percent`** *(default)*: Estimated target reflectance percentage.
- **`range-sigma-mm`**: Distance measurement standard deviation (sigma) in millimeters.
- **`nb-spads-enabled`**: Number of active SPADs enabled for the zone.

### Targets per Zone
Configures the maximum number of targets tracked per zone (default is 1). Choose at most one:
- **`targets-2`**: Up to 2 targets per zone.
- **`targets-3`**: Up to 3 targets per zone.
- **`targets-4`**: Up to 4 targets per zone.

### Plugins
Optional ST firmware plugins compiled into the driver:
- **`xtalk`**: Crosstalk calibration routines and margin configuration.
- **`detection-thresholds`**: Programmable per-zone detection checkers and interrupt triggers.
- **`motion-indicator`**: Motion detection across zones.
- **`plugins`**: Convenience meta-feature enabling `xtalk`, `detection-thresholds`, and `motion-indicator`.

### Other
- **`defmt`**: Derives `defmt::Format` on configuration enums, data structs, and error types.