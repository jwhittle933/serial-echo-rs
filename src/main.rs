#![no_std]
#![no_main]

use panic_halt as _;
use arduino_uno::prelude::*;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::hal::port::mode::Output;
use arduino_uno::pins;

#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();
    let pins = arduino_uno::pins!(peripherals);
    let mut led = pins.d13.into_output(&mut pins.ddr);

    loop {
        stutter_blink(&mut led, 25);
    }
}

fn stutter_blink(led: &mut PB5<Output>, times: usize) {
    (0..times).map(|i| i * 10).for_each(|i| {
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(i as u16);
    });
}
