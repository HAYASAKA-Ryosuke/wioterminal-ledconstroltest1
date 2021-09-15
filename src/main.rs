#![no_std]
#![no_main]
#![allow(dead_code)]


use panic_halt as _;
use wio_terminal as wio;
use wio::entry;

#[entry]
fn main() -> ! {
    unsafe {
        const PA_DIRSET: u32 = 0x41008008;
        *(PA_DIRSET as *mut u32) = 1 << 15;
        const PC_IN: u32 = 0x41008020 + 0x80 * 2;
        const PA_OUTSET: u32 = 0x41008018;
        const PA_OUTCLR: u32 = 0x41008014;
        const PC_PINCFG: u32 = 0x41008040 + (0x80 * 2) + (0x01 * 26);
        *(PC_PINCFG as *mut u8) = 2;
        loop {
            if *(PC_IN as *mut u32) & (1 << 26) != 0 {
                *(PA_OUTCLR as *mut u32) = 1 << 15;
            } else {
                *(PA_OUTSET as *mut u32) = 1 << 15;
            }
        }
    }
}
