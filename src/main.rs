#![no_std]
#![no_main]

use core::{
    fmt::Debug,
    ops::{BitAnd, Shl},
};

use esp32_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, IO};
use esp_backtrace as _;
use esp_println::println;
use num::Num;

const MATRIX_DELAY_NS: u32 = 100;

/// Returns the bit value at the given bit position.
fn get_bit_value<T: Num + BitAnd<Output = T> + Shl<u8, Output = T>>(value: T, bit: u8) -> bool {
    (value & (T::one() << bit)) != T::zero()
}

/// Write the value to the matrix data pin.
fn write_to_matrix<
    A: _embedded_hal_digital_v2_OutputPin,
    B: _embedded_hal_digital_v2_OutputPin,
    C: _embedded_hal_digital_v2_OutputPin,
>(
    delay: &Delay,
    data: u16,
    matrix_data_pin: &mut A,
    matrix_clock_pin: &mut B,
    matrix_chip_enable_pin: &mut C,
) where
    <A as esp32_hal::prelude::_embedded_hal_digital_v2_OutputPin>::Error: Debug,
    <B as esp32_hal::prelude::_embedded_hal_digital_v2_OutputPin>::Error: Debug,
    <C as esp32_hal::prelude::_embedded_hal_digital_v2_OutputPin>::Error: Debug,
{
    matrix_chip_enable_pin.set_low().unwrap();
    matrix_clock_pin.set_low().unwrap();
    for i in 0..16 {
        if get_bit_value(data, i) {
            matrix_data_pin.set_high().unwrap();
        } else {
            matrix_data_pin.set_low().unwrap();
        }
        delay.delay_nanos(MATRIX_DELAY_NS);
        matrix_clock_pin.set_high().unwrap();
        delay.delay_nanos(MATRIX_DELAY_NS);
        matrix_clock_pin.set_low().unwrap();
    }
    matrix_chip_enable_pin.set_high().unwrap();
    delay.delay_nanos(MATRIX_DELAY_NS);
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut matrix_power_pin = io.pins.gpio17.into_push_pull_output();
    let mut matrix_data_pin = io.pins.gpio5.into_push_pull_output();
    let mut matrix_chip_enable_pin = io.pins.gpio18.into_push_pull_output();
    let mut matrix_clock_pin = io.pins.gpio19.into_push_pull_output();

    matrix_power_pin.set_high().unwrap();

    let mut delay = Delay::new(&clocks);

    // write_to_matrix(
    //     &delay,
    //     2,
    //     &mut matrix_data_pin,
    //     &mut matrix_clock_pin,
    //     &mut matrix_chip_enable_pin,
    // );
    // matrix_chip_enable_pin.set_low().unwrap();

    // loop {}

    loop {
        for i in 0..u16::MAX {
            println!("{}", i);
            // for _ in 0..100 {
            write_to_matrix(
                &delay,
                i,
                &mut matrix_data_pin,
                &mut matrix_clock_pin,
                &mut matrix_chip_enable_pin,
            );
            // delay.delay_ms(10u32);
            // }
            delay.delay_ms(100u32);
        }
    }
}
