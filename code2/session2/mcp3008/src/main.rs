
extern crate spidev;
use std::{thread, time};
use spidev::{Spidev, SpidevOptions, SpidevTransfer, SPI_MODE_0};
fn create_spi() -> Spidev {
    let mut spi = Spidev::open("/dev/spidev0.0").unwrap();
    let options = SpidevOptions::new()
         .bits_per_word(8)
         .max_speed_hz(100_000)
         .mode(SPI_MODE_0)
         .build();
    spi.configure(&options).unwrap();
    spi
}


fn full_duplex(spi: &mut Spidev) -> u16 {
    // "write" transfers are also reads at the same time with
    // the read having the same length as the write
    let tx_buf = [0x01, 0x80, 0x0];
    let mut rx_buf = [0; 3];
    {
        let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
        spi.transfer(&mut transfer).unwrap();
    }
    ((rx_buf[1] as u16) << 8 | (rx_buf[2] as u16)) & 0x3ff
    
}

fn main() {
    let mut spi = create_spi();
    let delay = time::Duration::from_millis(500);
    loop {
        println!("{:?}", full_duplex(&mut spi));
        thread::sleep(delay);
    }
}
