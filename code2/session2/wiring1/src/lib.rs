pub mod cffi;

pub fn wiringPiSetup() {
    unsafe { cffi::wiringPiSetup(); }
}

pub fn pinMode(pin: i32, mode: u8) {
    unsafe { cffi::pinMode(pin, mode as i32); }
}

pub fn digitalWrite(pin: i32, val: u8) {
    unsafe { cffi::digitalWrite(pin, val as i32); }
}

pub fn digitalRead(pin: i32) -> i32 {
    unsafe { cffi::digitalRead(pin) }
}

pub fn delay(ms: u32) {
    unsafe { cffi::delay(ms); }
}


