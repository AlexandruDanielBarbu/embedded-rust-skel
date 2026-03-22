#![no_std]
#![no_main]

use cortex_m as _;
use cortex_m_rt::entry;
use defmt::{error, info};
// use RTT for defmt transport
use core::panic::PanicInfo;
use defmt_rtt as _;

#[entry]
fn main() -> ! {
    info!("This is my custom message. :)))))))))))))))))");
    // panic!("panic here");
    loop {
        // Your code goes here

        // For now, we just spin to satisfy the "!" return type
        cortex_m::asm::nop();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{:?}", info);
    loop {}
}
