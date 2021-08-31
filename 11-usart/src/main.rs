#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1};

use core::fmt::Write;

macro_rules! uprint {
    ( $serial: expr, $($arg: tt)* ) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()

    };
}

macro_rules! uprintln {
    ( $serial: expr, $fmt: expr ) => {
        uprint!($serial, concat!(fmt, "\n"))
    };
    ( $serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n"), $($arg)*)
    }
}

struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl core::fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        s.bytes().into_iter().for_each(|b| {
            while self.usart1.isr.read().txe().bit_is_clear() {}
            self.usart1.tdr.write(|w| w.tdr().bits(b as u16));
        });
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, mut itm) = aux11::init();

    let mut serial = SerialPort { usart1 };

    let mut ctr = 0;
    let mut buffer: [char; 32] = ['0'; 32];

    loop {
        while serial.usart1.isr.read().rxne().bit_is_clear() {}

        let byte = serial.usart1.rdr.read().rdr().bits();
        if byte != 13 {
            if ctr < 32 {
                let c = byte as u8 as char;
                uprint!(serial, "{}", c);
                buffer[ctr] = c;
                ctr += 1;
            }
        } else {
            uprint!(serial, "\n");
            for i in buffer[..ctr].iter().rev() {
                uprint!(serial, "{}", i);
            }
            uprint!(serial, "\n");
            ctr = 0;
        }
    }
}
