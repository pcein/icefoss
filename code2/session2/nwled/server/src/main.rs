
extern crate server;

use server::*;

use std::net::UdpSocket;

const PIN_LED1: i32 = 0;
const PIN_LED2: i32 = 2;
const PIN_INPUT: i32 = 3;

const addr: &str = "0.0.0.0:8000";

fn set_led_state(pin: i32, val: u8) {
    match val {
        0  => digitalWrite(pin, cffi::LOW),
        1  => digitalWrite(pin, cffi::HIGH),
         _    => (),
    }
}

fn main() {
    
    
    wiringPiSetup();
    pinMode(PIN_LED1, cffi::OUTPUT);
    pinMode(PIN_LED2, cffi::OUTPUT);
    pinMode(PIN_INPUT, cffi::INPUT);

    let mut socket = UdpSocket::bind(addr).unwrap();

    loop {
        let mut buf = [0; 2];

        let (amt, src) = socket.recv_from(&mut buf).unwrap();

        println!("{}, {}, {:?}", amt, src, buf);
   
        set_led_state(PIN_LED1, buf[0]);
        set_led_state(PIN_LED2, buf[1]);
    }
    

}
