//! FMC/FSMC timing

use super::fsmc;

/// Memory access modes
///
/// These define the general shape of a transaction and the meanings of some of the time fields.
/// Refer to the microcontroller reference manual for more details.
#[derive(Debug, Clone)]
pub enum AccessMode {
    ModeA,
    ModeB,
    ModeC,
    ModeD,
}

impl AccessMode {
    pub(crate) fn as_read_variant(&self) -> fsmc::btr::ACCMOD_A {
        use fsmc::btr::ACCMOD_A;
        match *self {
            AccessMode::ModeA => ACCMOD_A::A,
            AccessMode::ModeB => ACCMOD_A::B,
            AccessMode::ModeC => ACCMOD_A::C,
            AccessMode::ModeD => ACCMOD_A::D,
        }
    }
    pub(crate) fn as_write_variant(&self) -> fsmc::bwtr::ACCMOD_A {
        use fsmc::bwtr::ACCMOD_A;
        match *self {
            AccessMode::ModeA => ACCMOD_A::A,
            AccessMode::ModeB => ACCMOD_A::B,
            AccessMode::ModeC => ACCMOD_A::C,
            AccessMode::ModeD => ACCMOD_A::D,
        }
    }
}

/// Timing configuration for reading or writing
///
/// A `Timing` object can be created using `Timing::default()` or `Default::default()`.
///
/// The default timing uses access mode C and the slowest possible timings, for maximum
/// compatibility.
///
/// If the LCD controller and wiring allow, you can reduce the times to make transactions faster.
///
/// All time fields are in units of HCLK cycles.
#[derive(Debug, Clone)]
pub struct Timing {
    pub(crate) access_mode: AccessMode,
    pub(crate) bus_turnaround: u8,
    pub(crate) data: u8,
    pub(crate) address_hold: u8,
    pub(crate) address_setup: u8,
}

impl Default for Timing {
    /// Returns a conservative (slow) timing configuration with access mode C
    fn default() -> Self {
        Timing {
            access_mode: AccessMode::ModeC,
            bus_turnaround: 0,
            data: 255,
            address_hold: 0,
            address_setup: 1,
        }
    }
}

impl Timing {
    /// Sets the access mode
    pub fn access_mode(self, access_mode: AccessMode) -> Self {
        Timing {
            access_mode,
            ..self
        }
    }
    /// Sets the bus turnaround time, in units of HCLK cycles
    ///
    /// This corresponds to the BUSTURN field of FSMC_BTR or FSMC_BWTR.
    ///
    /// # Panics
    ///
    /// This function panics if bus_turnaround is greater than Timing::BUS_TURNAROUND_MAX.
    pub fn bus_turnaround(self, bus_turnaround: u8) -> Self {
        Timing {
            bus_turnaround,
            ..self
        }
    }
    /// Sets the data phase time, in units of HCLK cycles
    ///
    /// This corresponds to the DATAST field of FSMC_BTR or FSMC_BWTR.
    ///
    /// # Panics
    ///
    /// This function panics if data is less than Timing::DATA_MIN.
    pub fn data(self, data: u8) -> Self {
        Timing { data, ..self }
    }
    /// Sets the address hold phase time, in units of HCLK cycles
    ///
    /// This corresponds to the ADDHLD field of FSMC_BTR or FSMC_BWTR.
    ///
    /// # Panics
    ///
    /// This function panics if address_hold is less than Timing::ADDRESS_HOLD_MIN or greater than
    /// Timing::ADDRESS_HOLD_MAX.
    pub fn address_hold(self, address_hold: u8) -> Self {
        Timing {
            address_hold,
            ..self
        }
    }
    /// Sets the address setup phase time, in units of HCLK cycles
    ///
    /// This corresponds to the ADDSET field of FSMC_BTR or FSMC_BWTR.
    ///
    /// # Panics
    ///
    /// This function panics if address_setup is greater than Timing::ADDRESS_SETUP_MAX.
    pub fn address_setup(self, address_setup: u8) -> Self {
        Timing {
            address_setup,
            ..self
        }
    }
}
