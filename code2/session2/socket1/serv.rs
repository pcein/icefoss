
use std::net::UdpSocket;

const addr: &str = "127.0.0.1:8000";

fn main() {

    let mut socket = UdpSocket::bind(addr).unwrap();

    let mut buf = [0; 10];

    let (amt, src) = socket.recv_from(&mut buf).unwrap();

    println!("{}, {}, {:?}", amt, src, buf);
   
}
