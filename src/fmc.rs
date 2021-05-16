//! HAL for Flexible memory controller (FMC)
//!
//! See the stm32-fmc [usage guide](https://github.com/stm32-rs/stm32-fmc#usage)

// From stm32_fmc
use stm32_fmc::FmcPeripheral;
use stm32_fmc::{AddressPinSet, PinsSdram, Sdram, SdramChip, SdramPinSet, SdramTargetBank};

use crate::pac as stm32;
use crate::time::Hertz;

use crate::gpio::gpioa::PA7;
use crate::gpio::gpiob::{PB5, PB6, PB7};
use crate::gpio::gpioc::{PC0, PC2, PC3, PC4, PC5};
#[cfg(any(
    feature = "stm32f765",
    feature = "stm32f767",
    feature = "stm32f769",
    feature = "stm32f777",
    feature = "stm32f778",
    feature = "stm32f779"
))]
use crate::gpio::gpioc::{PC6, PC7, PC8};
use crate::gpio::gpiod::{
    PD0, PD1, PD10, PD11, PD12, PD13, PD14, PD15, PD3, PD4, PD5, PD6, PD7, PD8, PD9,
};
use crate::gpio::gpioe::{
    PE0, PE1, PE10, PE11, PE12, PE13, PE14, PE15, PE2, PE3, PE4, PE5, PE6, PE7, PE8, PE9,
};
use crate::gpio::gpiof::{PF0, PF1, PF11, PF12, PF13, PF14, PF15, PF2, PF3, PF4, PF5};
#[cfg(any(
    feature = "stm32f722",
    feature = "stm32f723",
    feature = "stm32f730",
    feature = "stm32f732",
    feature = "stm32f733"
))]
use crate::gpio::gpiog::PG11;
#[cfg(any(
    feature = "stm32f765",
    feature = "stm32f767",
    feature = "stm32f769",
    feature = "stm32f777",
    feature = "stm32f778",
    feature = "stm32f779"
))]
use crate::gpio::gpiog::PG6;
use crate::gpio::gpiog::{
    PG0, PG1, PG10, PG12, PG13, PG14, PG15, PG2, PG3, PG4, PG5, PG7, PG8, PG9,
};
use crate::gpio::gpioh::{PH10, PH11, PH12, PH13, PH14, PH15, PH2, PH3, PH5, PH6, PH7, PH8, PH9};
use crate::gpio::gpioi::{PI0, PI1, PI10, PI2, PI3, PI4, PI5, PI6, PI7, PI9};

#[cfg(any(
    feature = "stm32f765",
    feature = "stm32f767",
    feature = "stm32f769",
    feature = "stm32f777",
    feature = "stm32f778",
    feature = "stm32f779"
))]
use crate::gpio::AF9;
use crate::gpio::{Alternate, AF12};

/// Storage type for Flexible Memory Controller and its clocks
pub struct FMC {
    pub fmc: stm32::FMC,
    hclk: Hertz,
}

/// Extension trait for FMC controller
pub trait FmcExt: Sized {
    fn fmc(self, hclk: Hertz) -> FMC;

    /// A new SDRAM memory via the Flexible Memory Controller
    fn sdram<
        BANK: SdramPinSet,
        ADDR: AddressPinSet,
        PINS: PinsSdram<BANK, ADDR>,
        CHIP: SdramChip,
    >(
        self,
        pins: PINS,
        chip: CHIP,
        hclk: Hertz,
    ) -> Sdram<FMC, CHIP> {
        let fmc = self.fmc(hclk);
        Sdram::new(fmc, pins, chip)
    }

    /// A new SDRAM memory via the Flexible Memory Controller
    fn sdram_unchecked<CHIP: SdramChip, BANK: Into<SdramTargetBank>>(
        self,
        bank: BANK,
        chip: CHIP,
        hclk: Hertz,
    ) -> Sdram<FMC, CHIP> {
        let fmc = self.fmc(hclk);
        Sdram::new_unchecked(fmc, bank, chip)
    }
}

impl FmcExt for stm32::FMC {
    /// New FMC instance
    fn fmc(self, hclk: crate::time::Hertz) -> FMC {
        FMC { fmc: self, hclk }
    }
}

unsafe impl FmcPeripheral for FMC {
    const REGISTERS: *const () = stm32::FMC::ptr() as *const ();

    fn enable(&mut self) {
        // TODO : change it to something safe ...
        let rcc = unsafe { &(*stm32::RCC::ptr()) };

        // Enable FMC
        rcc.ahb3enr.modify(|_, w| w.fmcen().enabled());
        // Reset FMC
        rcc.ahb3rstr.modify(|_, w| w.fmcrst().reset());
        rcc.ahb3rstr.modify(|_, w| w.fmcrst().clear_bit());
    }

    fn source_clock_hz(&self) -> u32 {
        // FMC block is clocked by HCLK
        self.hclk.0
    }
}

macro_rules! pins {
    (FMC: $($pin:ident: [$($( #[ $pmeta:meta ] )* $inst:ty$(,)*)*])+) => {
        $(
            $(
                $( #[ $pmeta ] )*
                impl stm32_fmc::$pin for $inst {}
            )*
        )+
    }
}

pins! {
    FMC:
        A0: [
            PF0<Alternate<AF12>>
        ]
        A1: [
            PF1<Alternate<AF12>>
        ]
        A10: [
            PG0<Alternate<AF12>>
        ]
        A11: [
            PG1<Alternate<AF12>>
        ]
        A12: [
            PG2<Alternate<AF12>>
        ]
        A13: [
            PG3<Alternate<AF12>>
        ]
        A14: [
            PG4<Alternate<AF12>>
        ]
        A15: [
            PG5<Alternate<AF12>>
        ]
        A16: [
            PD11<Alternate<AF12>>
        ]
        A17: [
            PD12<Alternate<AF12>>
        ]
        A18: [
            PD13<Alternate<AF12>>
        ]
        A19: [
            PE3<Alternate<AF12>>
        ]
        A2: [
            PF2<Alternate<AF12>>
        ]
        A20: [
            PE4<Alternate<AF12>>
        ]
        A21: [
            PE5<Alternate<AF12>>
        ]
        A22: [
            PE6<Alternate<AF12>>
        ]
        A23: [
            PE2<Alternate<AF12>>
        ]
        A24: [
            PG13<Alternate<AF12>>
        ]
        A25: [
            PG14<Alternate<AF12>>
        ]
        A3: [
            PF3<Alternate<AF12>>
        ]
        A4: [
            PF4<Alternate<AF12>>
        ]
        A5: [
            PF5<Alternate<AF12>>
        ]
        A6: [
            PF12<Alternate<AF12>>
        ]
        A7: [
            PF13<Alternate<AF12>>
        ]
        A8: [
            PF14<Alternate<AF12>>
        ]
        A9: [
            PF15<Alternate<AF12>>
        ]
        BA0: [
            PG4<Alternate<AF12>>
        ]
        BA1: [
            PG5<Alternate<AF12>>
        ]
        CLK: [
            PD3<Alternate<AF12>>
        ]
        D0: [
            PD14<Alternate<AF12>>
        ]
        D1: [
            PD15<Alternate<AF12>>
        ]
        D10: [
            PE13<Alternate<AF12>>
        ]
        D11: [
            PE14<Alternate<AF12>>
        ]
        D12: [
            PE15<Alternate<AF12>>
        ]
        D13: [
            PD8<Alternate<AF12>>
        ]
        D14: [
            PD9<Alternate<AF12>>
        ]
        D15: [
            PD10<Alternate<AF12>>
        ]
        D16: [
            PH8<Alternate<AF12>>,
        ]
        D17: [
            PH9<Alternate<AF12>>,
        ]
        D18: [
            PH10<Alternate<AF12>>,
        ]
        D19: [
            PH11<Alternate<AF12>>,
        ]
        D2: [
            PD0<Alternate<AF12>>
        ]
        D20: [
            PH12<Alternate<AF12>>,
        ]
        D21: [
            PH13<Alternate<AF12>>,
        ]
        D22: [
            PH14<Alternate<AF12>>,
        ]
        D23: [
            PH15<Alternate<AF12>>,
        ]
        D24: [
            PI0<Alternate<AF12>>
        ]
        D25: [
            PI1<Alternate<AF12>>
        ]
        D26: [
            PI2<Alternate<AF12>>,
        ]
        D27: [
            PI3<Alternate<AF12>>
        ]
        D28: [
            PI6<Alternate<AF12>>
        ]
        D29: [
            PI7<Alternate<AF12>>
        ]
        D3: [
            PD1<Alternate<AF12>>
        ]
        D30: [
            PI9<Alternate<AF12>>
        ]
        D31: [
            PI10<Alternate<AF12>>
        ]
        D4: [
            PE7<Alternate<AF12>>
        ]
        D5: [
            PE8<Alternate<AF12>>
        ]
        D6: [
            PE9<Alternate<AF12>>
        ]
        D7: [
            PE10<Alternate<AF12>>
        ]
        D8: [
            PE11<Alternate<AF12>>
        ]
        D9: [
            PE12<Alternate<AF12>>
        ]
        DA0: [
            PD14<Alternate<AF12>>
        ]
        DA1: [
            PD15<Alternate<AF12>>
        ]
        DA10: [
            PE13<Alternate<AF12>>
        ]
        DA11: [
            PE14<Alternate<AF12>>
        ]
        DA12: [
            PE15<Alternate<AF12>>
        ]
        DA13: [
            PD8<Alternate<AF12>>
        ]
        DA14: [
            PD9<Alternate<AF12>>
        ]
        DA15: [
            PD10<Alternate<AF12>>
        ]
        DA2: [
            PD0<Alternate<AF12>>
        ]
        DA3: [
            PD1<Alternate<AF12>>
        ]
        DA4: [
            PE7<Alternate<AF12>>
        ]
        DA5: [
            PE8<Alternate<AF12>>
        ]
        DA6: [
            PE9<Alternate<AF12>>
        ]
        DA7: [
            PE10<Alternate<AF12>>
        ]
        DA8: [
            PE11<Alternate<AF12>>
        ]
        DA9: [
            PE12<Alternate<AF12>>
        ]
        INT: [
            PG7<Alternate<AF12>>,
            #[cfg(any(feature = "stm32f722",
                      feature = "stm32f723",
                      feature = "stm32f730",
                      feature = "stm32f732",
                      feature = "stm32f733"))]
            PG11<Alternate<AF12>>
        ]
        NBL0: [
            PE0<Alternate<AF12>>
        ]
        NBL1: [
            PE1<Alternate<AF12>>
        ]
        NBL2: [
            PI4<Alternate<AF12>>
        ]
        NBL3: [
            PI5<Alternate<AF12>>
        ]
        NCE: [
            #[cfg(any(feature = "stm32f765",
                      feature = "stm32f767",
                      feature = "stm32f769",
                      feature = "stm32f777",
                      feature = "stm32f778",
                      feature = "stm32f779"))]
            PC8<Alternate<AF9>>,
            PG9<Alternate<AF12>>
        ]
        NE1: [
            #[cfg(any(feature = "stm32f765",
                      feature = "stm32f767",
                      feature = "stm32f769",
                      feature = "stm32f777",
                      feature = "stm32f778",
                      feature = "stm32f779"))]
            PC7<Alternate<AF9>>,
            PD7<Alternate<AF12>>
        ]
        NE2: [
            #[cfg(any(feature = "stm32f765",
                      feature = "stm32f767",
                      feature = "stm32f769",
                      feature = "stm32f777",
                      feature = "stm32f778",
                      feature = "stm32f779"))]
            PC8<Alternate<AF9>>,
            PG9<Alternate<AF12>>
        ]
        NE3: [
            #[cfg(any(feature = "stm32f765",
                      feature = "stm32f767",
                      feature = "stm32f769",
                      feature = "stm32f777",
                      feature = "stm32f778",
                      feature = "stm32f779"))]
            PG6<Alternate<AF12>>,
            PG10<Alternate<AF12>>
        ]
        NE4: [
            PG12<Alternate<AF12>>
        ]
        NL: [
            PB7<Alternate<AF12>>
        ]
        NOE: [
            PD4<Alternate<AF12>>
        ]
        NWAIT: [
            #[cfg(any(feature = "stm32f765",
                      feature = "stm32f767",
                      feature = "stm32f769",
                      feature = "stm32f777",
                      feature = "stm32f778",
                      feature = "stm32f779"))]
            PC6<Alternate<AF9>>,
            PD6<Alternate<AF12>>
        ]
        NWE: [
            PD5<Alternate<AF12>>
        ]
        SDCKE0: [
            PC3<Alternate<AF12>>,
            PC5<Alternate<AF12>>,
            PH2<Alternate<AF12>>
        ]
        SDCKE1: [
            PB5<Alternate<AF12>>,
            PH7<Alternate<AF12>>
        ]
        SDCLK: [
            PG8<Alternate<AF12>>
        ]
        SDNCAS: [
            PG15<Alternate<AF12>>
        ]
        SDNE0: [
            PC2<Alternate<AF12>>,
            PC4<Alternate<AF12>>,
            PH3<Alternate<AF12>>
        ]
        SDNE1: [
            PB6<Alternate<AF12>>,
            PH6<Alternate<AF12>>
        ]
        SDNRAS: [
            PF11<Alternate<AF12>>
        ]
        SDNWE: [
            PA7<Alternate<AF12>>,
            PC0<Alternate<AF12>>,
            PH5<Alternate<AF12>>
        ]
}
