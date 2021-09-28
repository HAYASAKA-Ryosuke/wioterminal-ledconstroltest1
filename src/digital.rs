#[derive(Debug)]
pub enum Error {
    MissingPin,
}

pub enum Group {
    Group1 = 0,
    Group2,
    Group3,
    Group4,
}

pub enum DigitalOutputMode {
    On,
    Off
}

pub enum DigitalReadMode {
    On,
    Off
}

pub fn digital_output_mode(group: Group, pin: u32, mode: DigitalOutputMode) -> Result<(), Error>{
    if pin > 32 {
        return Err(Error::MissingPin);
    }
    let dirset = 0x41008008 + (0x80 * (group as u32) );
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

/// 出力可能なピンをHighにする
/// 
/// 無効なピンを指定されたときにはErrorを返却
/// ## Examples
/// PA15のピンをHighにするとき
/// ```
/// digital_output_mode(Group::group1, 15, DigitalReadMode::On).unwrap();
/// digital_high(Group::group1, 15).unwrap()
/// ```
pub fn digital_high(group: Group, pin: u32) -> Result<(), Error> {
    if pin > 32 {
        return Err(Error::MissingPin);
    }
    let outset = 0x41008018 + (0x80 * (group as u32));
    unsafe {
        *(outset as *mut u32) |= 1 << pin; 
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
pub fn digital_low(group: Group, pin: u32) -> Result<(), Error> {
    if pin > 32 {
        return Err(Error::MissingPin);
    }
    let outclr = 0x41008014 + (0x80 * (group as u32));
    unsafe {
        *(outclr as *mut u32) |= 1 << pin;
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
pub fn digital_read_mode(group: Group, pin: u32, mode: DigitalReadMode) -> Result<(), Error> {
    if pin > 32 {
        return Err(Error::MissingPin);
    }
    let pin_config = 0x41008040 + (0x80 * (group as u32)) + (0x01 * pin);
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
pub fn digital_pin_read(group: Group, pin: u32) -> Result<bool, Error> {
    if pin > 32 {
        return Err(Error::MissingPin);
    }
    let pin_in = 0x41008020 + (0x80 * (group as u32));
    unsafe {
        Ok(*(pin_in as *mut u32) & (1 << pin) == 0)
    }
    
}