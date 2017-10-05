extern crate gpioctrl;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;
use gpioctrl::*;

const PIN_LED1:i32 = 0;
const PIN_LED2:i32 = 2;
const PIN_INPUT:i32 = 3;

fn led1_ctrl(status: &str) {
    match status {
        "on"  => digitalWrite(PIN_LED1, cffi::HIGH),
        "off" => digitalWrite(PIN_LED1, cffi::LOW),
        _     => (),
    }
}

fn led2_ctrl(status: &str) {
    match status {
        "on"  => digitalWrite(PIN_LED2, cffi::HIGH),
        "off" => digitalWrite(PIN_LED2, cffi::LOW),
        _     => (),
    }
}


fn init_ports() {
    wiringPiSetup();
    pinMode(PIN_LED1, cffi::OUTPUT);   
    pinMode(PIN_LED2, cffi::OUTPUT);   
    pinMode(PIN_INPUT, cffi::INPUT);
}
   

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long = "led1", help = "turn led1 ON/OFF")]
    led1: Option<String>,

    #[structopt(long = "led2", help = "turn led2 ON/OFF")]
    led2: Option<String>,

    #[structopt(long="readinput", help = "read status of input pin")]
    readinput: bool,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    
    init_ports();
    if let Some(status) = opt.led1 {
        led1_ctrl(&status);
    } 
    if let Some(status) = opt.led2 {
        led2_ctrl(&status);  
    }
    if opt.readinput {
        println!("{}", digitalRead(PIN_INPUT));
    }
}
