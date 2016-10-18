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

#[repr(usize)]
enum GPIO {
    GPIOA = 0,
    GPIOB = 1*1024,
    GPIOC = 2*1024,
    GPIOD = 3*1024,
    GPIOE = 4*1024,
    GPIOF = 5*1024,
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
