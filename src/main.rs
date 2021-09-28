#![no_std]
#![no_main]
#![allow(dead_code)]
mod digital;


use panic_halt as _;
use wio_terminal as wio;
use wio::entry;

#[entry]
fn main() -> ! {
    // PA15(LED)を出力に設定
    digital::digital_output_mode(digital::Group::Group1, 15, digital::DigitalOutputMode::On).unwrap();
    // PC26(button1)を入力に設定
    digital::digital_read_mode(digital::Group::Group3, 26, digital::DigitalReadMode::On).unwrap();

    loop {
        // PC26ピン(button1)が入力されていればLED ON
        if digital::digital_pin_read(digital::Group::Group3, 26).unwrap() {
            // LED ON(PA15)
            digital::digital_high(digital::Group::Group1, 15).unwrap();
        } else {
            // LED OFF(PA15)
            digital::digital_low(digital::Group::Group1, 15).unwrap();
        }
    }
}
