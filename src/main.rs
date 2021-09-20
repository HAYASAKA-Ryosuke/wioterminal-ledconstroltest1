#![no_std]
#![no_main]
#![allow(dead_code)]


use panic_halt as _;
use wio_terminal as wio;
use wio::entry;

#[derive(Debug)]
enum Error {
    MissingPin,
}

enum Group {
    Group1,
    Group2,
    Group3,
    Group4,
}

enum DigitalOutputMode {
    On,
    Off
}

enum DigitalReadMode {
    On,
    Off
}


fn digital_output_mode(group: Group, pin: u32, mode: DigitalOutputMode) -> Result<(), Error>{
    if pin > 32 {
        return Err(Error::MissingPin);
    }
    let group_number = match group {
        Group::Group4 => 3,
        Group::Group3 => 2,
        Group::Group2 => 1,
        Group::Group1 => 0,
    };
    let dirset = 0x41008008 + (0x80 * group_number);
    match mode {
        DigitalOutputMode::Off => {
            unsafe {
                *(dirset as *mut u32) &= !(1 << pin);
            }
        },
        DigitalOutputMode::On => {
            unsafe {
               *(dirset as *mut u32) |= 1 << pin;
            }
        }
    }
    Ok(())
}

fn digital_high(group: Group, pin: u32) -> Result<(), Error> {
    if pin > 32 {
        return Err(Error::MissingPin);
    }
    let group_number = match group {
        Group::Group4 => 3,
        Group::Group3 => 2,
        Group::Group2 => 1,
        Group::Group1 => 0,
    };
    let outset = 0x41008018 + (0x80 * group_number);
    unsafe {
        *(outset as *mut u32) |= 1 << pin; 
    }
    Ok(())
}

fn digital_low(group: Group, pin: u32) -> Result<(), Error> {
    if pin > 32 {
        return Err(Error::MissingPin);
    }
    let group_number = match group {
        Group::Group4 => 3,
        Group::Group3 => 2,
        Group::Group2 => 1,
        Group::Group1 => 0,
    };
    let outclr = 0x41008014 + (0x80 * group_number);
    unsafe {
        *(outclr as *mut u32) |= 1 << pin;
    }
    Ok(())
}

fn digital_read_mode(group: Group, pin: u32, mode: DigitalReadMode) -> Result<(), Error> {
    if pin > 32 {
        return Err(Error::MissingPin);
    }
    let group_number = match group {
        Group::Group4 => 3,
        Group::Group3 => 2,
        Group::Group2 => 1,
        Group::Group1 => 0,
    };
    let pin_config = 0x41008040 + (0x80 * group_number) + (0x01 * pin);
    match mode {
        DigitalReadMode::On => {
            unsafe {
                // INENビット(Input Enable)を有効
                *(pin_config as *mut u8) |= 2;
            }
        },
        DigitalReadMode::Off => {
            unsafe {
                // INENビット(Input Enable)を無効
                *(pin_config as *mut u8) &= !2;
            }
        }
    }
    Ok(())
}

fn digital_pin_read(group: Group, pin: u32) -> Result<bool, Error> {
    if pin > 32 {
        return Err(Error::MissingPin);
    }
    let group_number = match group {
        Group::Group4 => 3,
        Group::Group3 => 2,
        Group::Group2 => 1,
        Group::Group1 => 0,
    };
    let pin_in = 0x41008020 + (0x80 * group_number);
    unsafe {
        Ok(*(pin_in as *mut u32) & (1 << pin) != 0)
    }
}

#[entry]
fn main() -> ! {
    // PA15(LED)を出力に設定
    digital_output_mode(Group::Group1, 15, DigitalOutputMode::On).unwrap();
    // PC26(button1)を入力に設定
    digital_read_mode(Group::Group3, 26, DigitalReadMode::On).unwrap();

    loop {
        // PC26ピン(button1)が入力されていればLED ON
        if digital_pin_read(Group::Group3, 26).unwrap() {
            // LED OFF(PA15)
            digital_low(Group::Group1, 15).unwrap();
        } else {
            // LED ON(PA15)
            digital_high(Group::Group1, 15).unwrap();
        }
    }
}
