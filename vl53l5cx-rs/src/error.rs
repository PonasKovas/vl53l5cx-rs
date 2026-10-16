/// Driver error. `E` is the I2C error type of the bus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<E> {
    /// The I2C bus returned an error during a transfer.
    I2c(E),
    /// The sensor did not answer in time (ULD status 1).
    Timeout,
    /// The ranging frame was corrupted (ULD status 2).
    CorruptedFrame,
    /// Checksum failure on the firmware/data (ULD status 3).
    CrcChecksum,
    /// A parameter was rejected (ULD status 127, or bad address).
    InvalidParam,
    /// Any other (possibly OR-combined) ULD status code.
    Uld(u8),
}
