#![no_std]
#![no_main]
#![allow(dead_code)]


use panic_halt as _;
use wio_terminal as wio;
use wio::entry;

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

#[derive(Debug)]
enum DigitalOutputModeError {
    MissingPin,
}

#[derive(Debug)]
enum DigitalHighLowError {
    MissingPin,
}

enum DigitalReadMode {
    On,
    Off
}

#[derive(Debug)]
enum DigitalReadModeError {
    MissingPin,
}

#[derive(Debug)]
enum DigitalPinReadError {
    MissingPin,
}

fn digital_output_mode(group: Group, pin: u32, mode: DigitalOutputMode) -> Result<(), DigitalOutputModeError>{
    if pin > 32 {
        return Err(DigitalOutputModeError::MissingPin);
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

fn digital_high(group: Group, pin: u32) -> Result<(), DigitalHighLowError> {
    if pin > 32 {
        return Err(DigitalHighLowError::MissingPin);
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

fn digital_low(group: Group, pin: u32) -> Result<(), DigitalHighLowError> {
    if pin > 32 {
        return Err(DigitalHighLowError::MissingPin);
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

fn digital_read_mode(group: Group, pin: u32, mode: DigitalReadMode) -> Result<(), DigitalReadModeError> {
    if pin > 32 {
        return Err(DigitalReadModeError::MissingPin);
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

fn digital_pin_read(group: Group, pin: u32) -> Result<bool, DigitalPinReadError> {
    if pin > 32 {
        return Err(DigitalPinReadError::MissingPin);
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
        const PA_OUTSET: u32 = 0x41008018;
        const PA_OUTCLR: u32 = 0x41008014;

        digital_output_mode(Group::Group1, 15, DigitalOutputMode::On).unwrap();

        // PCグループのピンの入力をあつかうレジスタを選択
        //const PC_IN: u32 = 0x41008020 + 0x80 * 2; // PCグループ(0x80 * 2)

        digital_read_mode(Group::Group3, 26, DigitalReadMode::On).unwrap();

        loop {
            // PC26ピン(button1)が入力されていればLED ON
            if digital_pin_read(Group::Group3, 26).unwrap() {
                // *(PA_OUTCLR as *mut u32) = 1 << 15; // LED OFF(PA15)
                digital_low(Group::Group1, 15).unwrap();
            } else {
                // *(PA_OUTSET as *mut u32) = 1 << 15;  // LED ON(PA15)
                digital_high(Group::Group1, 15).unwrap();
            }
        }
}
