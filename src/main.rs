#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::i2c::I2c;
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use embassy_time::{Duration, Timer, with_timeout};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut config = i2c::Config::default();
    config.sda_pullup = true;
    config.scl_pullup = true;

    let mut i2c = I2c::new(
        p.I2C1,
        p.PB6, // SCL
        p.PB7, // SDA
        Irqs,
        p.GPDMA1_CH0,
        p.GPDMA1_CH1,
        config,
    );

    info!("Incep I2C scan...");

    for addr in 0x08u8..=0x77 {
        let mut buf = [0u8; 1];
        match with_timeout(Duration::from_millis(10), i2c.read(addr, &mut buf)).await {
            Ok(Ok(_)) => info!("Gasit dispozitiv la adresa: 0x{:02X}", addr),
            _ => {}
        }
    }

    info!("Scan complet.");

    loop {
        Timer::after_secs(60).await;
    }
}
