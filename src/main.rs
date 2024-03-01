#![no_std]
#![no_main]

use esp32_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, IO};
use esp_backtrace as _;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio12.into_push_pull_output();

    led.set_high().unwrap();

    let mut delay = Delay::new(&clocks);

    loop {
        led.toggle().unwrap();
        delay.delay_ms(1000u32);
    }
}
