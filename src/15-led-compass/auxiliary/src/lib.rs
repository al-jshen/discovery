//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;
pub use f3::{
    hal::{
        delay::Delay, gpio::gpiob::PB6, gpio::gpiob::PB7, gpio::AF4, prelude, stm32f30x::i2c1,
        stm32f30x::I2C1,
    },
    led::{Direction, Leds},
};
pub use lsm303agr::UnscaledMeasurement;
use lsm303agr::{interface::I2cInterface, mode::MagContinuous, Lsm303agr};

use f3::hal::{i2c::I2c, prelude::*, stm32f30x};

pub type LSM303AGR = Lsm303agr<I2cInterface<I2c<I2C1, (PB6<AF4>, PB7<AF4>)>>, MagContinuous>;

pub fn init() -> (Leds, LSM303AGR, Delay, ITM) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let leds = Leds::new(gpioe);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);

    let mut lsm = Lsm303agr::new_with_i2c(i2c);
    lsm.init().unwrap();

    lsm.set_mag_odr(lsm303agr::MagOutputDataRate::Hz10).unwrap();
    let lsm303agr = lsm.into_mag_continuous().ok().unwrap();

    let delay = Delay::new(cp.SYST, clocks);

    (leds, lsm303agr, delay, cp.ITM)
}
