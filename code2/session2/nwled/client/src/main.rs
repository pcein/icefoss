

extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;
use std::net::UdpSocket;

const PIN_LED1: i32 = 0;
const PIN_LED2: i32 = 2;
const PIN_INPUT: i32 = 3;

fn set_led_state(buf: &mut [u8], pin: usize, val: &str) {
    match val {
        "on"  => buf[pin] = 1,
        "off" => buf[pin] = 0,
         _    => (),
    }
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long = "led1", help = "turn led1 ON/OFF")]
    led1: Option<String>,

    #[structopt(long = "led2", help = "turn led2 ON/OFF")]
    led2: Option<String>,

    #[structopt(long="remoteaddr", help = "remote IP")]
    remoteaddr: String,
}

fn main() {
    
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let mut buf = [2;2];
 
    if let Some(state) = opt.led1 {
        set_led_state(&mut buf, 0, &state);
    }
    
    if let Some(state) = opt.led2 {
        set_led_state(&mut buf, 1, &state);
    }

    let serv_addr = opt.remoteaddr;

    let mut socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    socket.send_to(&buf, serv_addr);
    

}
