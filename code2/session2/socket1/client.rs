
use std::net::UdpSocket;

const serv_addr: &str = "127.0.0.1:8000";
const cli_addr: &str = "0.0.0.0:0";

fn main() {

    let mut socket = UdpSocket::bind(cli_addr).unwrap();

    let mut buf = [4; 10];

    socket.send_to(&buf, serv_addr);
}
