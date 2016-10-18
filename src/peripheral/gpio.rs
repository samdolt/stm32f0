//! # GPIO Registers definitions
//!
//! Source:
//!
//! * RM0360 - STM32F030x4/6/8/C and STM32F070x6/B (Rev 3, May 2015)
//!   - Page 40 for memory adress
//!   - Page 143 for GPIO register list and offset
//! * RM0091 - STM32F0x1/STM32F0x2/STM32F0x8  (Rev 8, July 2015)
//!   - Page 46 for memory adress
//!   - Page 171 for GPIO register list and offset
//!
//! # Safety
//!
//! Using function in these module are unsafe:
//!
//! - Getting multiple mutable reference is possible
//! - Changing reserved part of some register is allowed (Undefined Behavor)


use volatile_register::{RO, RW, WO};

const PORT_BASE: usize = 0x4800_0000;
const PORT_SIZE: usize = 1024;

#[repr(usize)]
pub enum Ports {
    #[cfg(feature = "GPIOA")]
    PortA = 0,

    #[cfg(feature = "GPIOB")]
    PortB = 1,

    #[cfg(feature = "GPIOC")]
    PortC = 2,

    #[cfg(feature = "GPIOD")]
    PortD = 3,

    #[cfg(feature = "GPIOE")]
    PortE = 4,

    #[cfg(feature = "GPIOF")]
    PortF = 5,
}

#[repr(u16)]
enum Pins {
    Pin0 = 0b0000000000000001,
    Pin1 = 0b0000000000000010,
    Pin2 = 0b0000000000000100,
    Pin3 = 0b0000000000001000,
    Pin4 = 0b0000000000010000,
    Pin5 = 0b0000000000100000,
    Pin6 = 0b0000000001000000,
    Pin7 = 0b0000000010000000,
    Pin8 = 0b0000000100000000,
    Pin9 = 0b0000001000000000,
    Pin10 = 0b0000010000000000,
    Pin11 = 0b0000100000000000,
    Pin12 = 0b0001000000000000,
    Pin13 = 0b0010000000000000,
    Pin14 = 0b0100000000000000,
    Pin15 = 0b1000000000000000,
}


#[repr(C)]
pub struct GPIO {
    MODER: RW<u32>,
    OTYPER: RW<u32>,
    OSPEEDR: RW<u32>,
    PUPDR: RW<u32>,
    IDR: RO<u32>,
    ODR: RW<u32>,
    BSRR: WO<u32>,
    LCKR: RW<u32>,
    AFRL: RW<u32>,
    AFRH: RW<u32>,
    BRR: WO<u32>,
}

unsafe fn get_gpio2(port: Ports) -> &'static mut GPIO {
    let adress: usize = port * PORT_SIZE + PORT_BASE;
    &mut *(adress as *mut GPIO)

}

unsafe fn get_gpio(adress: usize) -> &'static mut GPIO {
    &mut *(adress as *mut GPIO)
}

#[cfg(feature = "GPIOA")]
pub unsafe fn GPIOA() -> &'static mut GPIO {
    get_gpio(0x4800_0000)
}

#[cfg(feature = "GPIOB")]
pub unsafe fn GPIOB() -> &'static mut GPIO {
    get_gpio(0x4800_0400)
}

#[cfg(feature = "GPIOC")]
pub unsafe fn GPIOC() -> &'static mut GPIO {
    get_gpio(0x4800_0800)
}

#[cfg(feature = "GPIOD")]
pub unsafe fn GPIOD() -> &'static mut GPIO {
    get_gpio(0x4800_0C00)
}

#[cfg(feature = "GPIOE")]
pub unsafe fn GPIOE() -> &'static mut GPIO {
    get_gpio(0x4800_1000)
}

#[cfg(feature = "GPIOF")]
pub unsafe fn GPIOF() -> &'static mut GPIO {
    get_gpio(0x4800_1400)
}
