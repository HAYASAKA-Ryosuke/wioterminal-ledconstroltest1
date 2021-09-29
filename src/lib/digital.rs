use crate::{lib};

#[derive(Debug)]
pub enum Error {
    MissingPin,
}

pub enum DigitalOutputMode {
    On,
    Off
}

pub enum DigitalReadMode {
    On,
    Off
}


pub fn digital_output_mode(pin: &lib::pin::PinGroup, mode: DigitalOutputMode) -> Result<(), Error>{
    if pin.number > 32 {
        return Err(Error::MissingPin);
    }
    let dirset = 0x41008008 + (0x80 * (pin.group as u32) );
    match mode {
        DigitalOutputMode::Off => {
            unsafe {
                *(dirset as *mut u32) &= !(1 << pin.number);
            }
        },
        DigitalOutputMode::On => {
            unsafe {
               *(dirset as *mut u32) |= 1 << pin.number;
            }
        }
    }
    Ok(())
}

/// 出力可能なピンをHighにする
/// 
/// 無効なピンを指定されたときにはErrorを返却
/// ## Examples
/// PA15のピンをHighにするとき
/// ```
/// digital_output_mode(Group::group1, 15, DigitalReadMode::On).unwrap();
/// digital_high(Group::group1, 15).unwrap()
/// ```
pub fn digital_high(pin: &lib::pin::PinGroup) -> Result<(), Error> {
    if pin.number > 32 {
        return Err(Error::MissingPin);
    }
    let outset = 0x41008018 + (0x80 * (pin.group as u32));
    unsafe {
        *(outset as *mut u32) |= 1 << pin.number;
    }
    Ok(())
}

/// 出力可能なピンをLowにする
/// 
/// 無効なピンを指定されたときにはErrorを返却
/// ## Examples
/// PA15のピンをLowにするとき
/// ```
/// digital_output_mode(Group::group1, 15, DigitalReadMode::On).unwrap();
/// digital_low(Group::group1, 15).unwrap()
/// ```
pub fn digital_low(pin: &lib::pin::PinGroup) -> Result<(), Error> {
    if pin.number > 32 {
        return Err(Error::MissingPin);
    }
    let outclr = 0x41008014 + (0x80 * (pin.group as u32));
    unsafe {
        *(outclr as *mut u32) |= 1 << pin.number;
    }
    Ok(())
}

/// ピン入力を検出できるようにする
/// 
/// 無効なピンを指定されたときにはErrorを返却
/// ## Examples
/// ```
/// digital_read_mode(Group::group3, 26, DigitalReadMode::On).unwrap();
/// digital_pin_read(Group::group3, 26).unwrap()
/// ```
pub fn digital_read_mode(pin: &lib::pin::PinGroup, mode: DigitalReadMode) -> Result<(), Error> {
    if pin.number > 32 {
        return Err(Error::MissingPin);
    }
    let pin_config = 0x41008040 + (0x80 * (pin.group as u32)) + (0x01 * pin.number);
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

/// ピン入力状態を取得
/// 
/// 入力状態のときtrue,未入力のときfalse
/// 
/// 無効なピンを指定されたときにはErrorを返却
/// 
/// ## Examples
/// ```
/// digital_read_mode(Group::group3, 26, DigitalReadMode::On).unwrap();
/// digital_pin_read(Group::group3, 26).unwrap()
/// ```
pub fn digital_pin_read(pin: &lib::pin::PinGroup) -> Result<bool, Error> {
    if pin.number > 32 {
        return Err(Error::MissingPin);
    }
    let pin_in = 0x41008020 + (0x80 * (pin.group as u32));
    unsafe {
        Ok(*(pin_in as *mut u32) & (1 << pin.number) == 0)
    }
    
}