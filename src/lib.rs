#![no_std]



#[macro_use]
extern crate bitflags;

extern crate volatile_register;

#[allow(non_snake_case)]
pub mod peripheral;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
