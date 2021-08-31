#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*};

#[entry]
fn main() -> ! {
    let (_leds, mut lsm, mut delay, mut itm) = aux15::init();

    loop {
        if lsm.mag_status().unwrap().xyz_new_data {
            iprintln!(&mut itm.stim[0], "{:?}", lsm.mag_data().unwrap());
        }

        delay.delay_ms(1000 as u16);
    }
}
