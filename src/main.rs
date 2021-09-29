#![no_std]
#![no_main]
#![allow(dead_code)]
mod lib;


use panic_halt as _;
use wio_terminal as wio;

use wio::entry;

#[entry]
fn main() -> ! {
    let pins = lib::pin::init_pins();
    // PC5(LCD light)を出力に設定
    lib::digital::digital_output_mode(&pins.C5, lib::digital::DigitalOutputMode::On).unwrap();
    // PC26(button1)を入力に設定
    lib::digital::digital_read_mode(&pins.C26, lib::digital::DigitalReadMode::On).unwrap();

    loop {
        // PC26ピン(button1)が入力されていればLED ON
        if lib::digital::digital_pin_read(&pins.C26).unwrap() {
            // lcd light ON(PC5)
            lib::digital::digital_high(&pins.C5).unwrap();
        } else {
            // lcd light OFF(PC5)
            lib::digital::digital_low(&pins.C5).unwrap();
        }
    }
}
