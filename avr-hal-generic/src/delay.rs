/*
//! Delay implementations

use core::marker;
use embedded_hal::delay::DelayNs;

#[cfg(all(target_arch = "avr", avr_hal_asm_macro))]
use core::arch::asm;

/// A busy-loop delay implementation
///
/// # Example
/// ```rust
/// // Instead of arduino_hal below you may also use a different
/// // HAL based on avr_hal_generic like attiny_hal or atmega_hal
/// // depending on actual hardware. For example:
/// //
/// // use attiny_hal as hal;
///
/// use arduino_hal as hal;
/// use embedded_hal_v0::prelude::*;
///
/// let mut delay = embedded_hal_v0::delay::Delay::<hal::clock::MHz16>::new();
///
/// // Wait 1 second
/// delay.delay_ms(1000);
/// ```
///
/// # Warning
/// The delay is not accurate for values above 4095µs because of a loop whose
/// overhead is not accounted for.  This will be fixed in a future version.
#[derive(Debug, Clone, Copy)]
pub struct Delay<SPEED> {
    _speed: marker::PhantomData<SPEED>,
}

impl<SPEED> Delay<SPEED> {
    pub fn new() -> Delay<SPEED> {
        Delay {
            _speed: marker::PhantomData,
        }
    }
}

// based on https://github.com/arduino/ArduinoCore-avr/blob/master/cores/arduino/wiring.c

cfg_if::cfg_if! {
    if #[cfg(all(target_arch = "avr", avr_hal_asm_macro))] {
        #[allow(unused_assignments)]
        fn busy_loop(mut c: u16) {
            unsafe {
                asm!(
                    "1:",
                    "sbiw {c}, 1",
                    "brne 1b",
                    c = inout(reg_iw) c,
                );
            }
        }
    } else if #[cfg(target_arch = "avr")] {
        #[allow(unused_assignments)]
        fn busy_loop(mut c: u16) {
            unsafe {
                llvm_asm!("1: sbiw $0,1\n\tbrne 1b"
                     : "=w"(c)
                     : "0"(c)
                     :
                     : "volatile"
                 );
            }
        }
    } else {
        fn busy_loop(_c: u16) {
            unimplemented!("Implementation is only available for avr targets!")
        }
    }
}
*/
