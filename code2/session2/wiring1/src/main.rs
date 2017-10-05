
extern crate wiring1;

use wiring1::*;

const PIN_LED: i32 = 0;
const PIN_INPUT: i32 = 3;

fn main() {
    println!("Hello, world!");
    wiringPiSetup();
    pinMode(PIN_LED, cffi::OUTPUT);
    pinMode(PIN_INPUT, cffi::INPUT);

    loop {
        digitalWrite(PIN_LED, cffi::HIGH);
        println!("input = {}", digitalRead(PIN_INPUT));
        delay(500);
        digitalWrite(PIN_LED, cffi::LOW);
        println!("input = {}", digitalRead(PIN_INPUT));
        delay(500);
    }
}
