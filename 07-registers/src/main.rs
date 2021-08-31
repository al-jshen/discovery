#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln, ITM};
use core::ptr;

#[entry]
fn main() -> ! {
    let (mut itm, gpioe) = aux7::init();

    gpioe.bsrr.write(|w| w.bs9().set_bit());
    gpioe.bsrr.write(|w| w.bs11().set_bit());
    gpioe.bsrr.write(|w| w.br9().set_bit());
    gpioe.bsrr.write(|w| w.br11().set_bit());

    loop {}
}
