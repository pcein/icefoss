
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // The 'Result' value returned by write_all is ignored.
    let mut f1 = File::create("/sys/class/gpio/export").unwrap();
    f1.write_all(b"0");

    let mut f2 = File::create("/sys/class/gpio/gpio0/direction").unwrap();
    f2.write_all(b"out");
        
    let mut f3 = File::create("/sys/class/gpio/gpio0/value").unwrap();
    f3.write_all(b"1");

}

