#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_extern_crates)] //  bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

#[allow(unused_imports)]
use lsm303agr::*;

use cortex_m::{asm::bkpt, iprint, iprintln};
use cortex_m_rt::entry;

use lsm303agr::Lsm303agr;
use stm32f3_discovery::stm32f3xx_hal::{i2c::I2c, prelude::*, stm32};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut itm = cp.ITM;
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let i2c = I2c::new(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);

    let mut lsm = Lsm303agr::new_with_i2c(i2c);
    lsm.init().unwrap();
    lsm.set_mag_odr(MagOutputDataRate::Hz10).unwrap();

    loop {
        if lsm.mag_status().unwrap().xyz_new_data {
            iprintln!(&mut itm.stim[0], "{:?}", lsm.mag_data().unwrap());
        }
    }
}
