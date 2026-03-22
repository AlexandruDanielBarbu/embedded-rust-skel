#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::{error, info};
// use RTT for defmt transport
use core::panic::PanicInfo;
use defmt_rtt as _;

#[entry]
fn main() -> ! {
    info!("Device has started");
    panic!("panic here");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{:?}", info);
    loop {}
}
