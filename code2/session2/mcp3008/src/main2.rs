
extern crate spidev;
use std::io;
use spidev::{Spidev, SpidevOptions, SpidevTransfer, SPI_MODE_0};

fn create_spi() -> io::Result<Spidev> {
    let mut spi = try!(Spidev::open("/dev/spidev0.0"));
    let options = SpidevOptions::new()
         .bits_per_word(8)
         .max_speed_hz(100_000)
         .mode(SPI_MODE_0)
         .build();
    try!(spi.configure(&options));
    Ok(spi)
}


/// Perform full duplex operations using Ioctl
fn full_duplex(spi: &mut Spidev) -> io::Result<()> {
    // "write" transfers are also reads at the same time with
    // the read having the same length as the write
    let tx_buf = [0x01, 0x80, 0x0];
    let mut rx_buf = [0; 3];
    {
        let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
        try!(spi.transfer(&mut transfer));
    }
    println!("{:?}", rx_buf);
    Ok(())
}

fn main() {
    let mut spi = create_spi().unwrap();
    println!("{:?}", full_duplex(&mut spi).unwrap());
}
