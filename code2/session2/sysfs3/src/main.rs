
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};

fn main() {
    let mut f1 = File::create("/sys/class/gpio/export").unwrap();
    f1.write_all(b"0");

    let mut f2 = File::create("/sys/class/gpio/gpio0/direction").unwrap();
    f2.write_all(b"out");
        
    let mut f3 = File::create("/sys/class/gpio/gpio0/value").unwrap();

    // 500 milli seconds
    let delay = time::Duration::from_millis(500);
    // thread::sleep(delay) will sleep for 500ms.

    loop {
        // write a "1" to f3, delay for .5 seconds,
        // write a "0"to f3, delay for .5 seconds
    }
    
}

