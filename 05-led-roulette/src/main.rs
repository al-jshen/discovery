#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, DelayMs, OutputSwitch};
use volatile::Volatile;

#[entry]
fn main() -> ! {
    let (mut delay, mut leds) = aux5::init();

    let t = 50_u16;

    loop {
        for i in 0..8 {
            leds[i].on().ok();
            delay.delay_ms(t);

            leds[(i + 1) % 8].on().ok();
            leds[i].off().ok();
            delay.delay_ms(t);
        }
    }
}
