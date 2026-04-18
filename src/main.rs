#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_time::Timer;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_stm32::init(Default::default());

    let mut led = Output::new(peripherals.PC7, Level::Low, Speed::Low);
    let mut button = Input::new(peripherals.PA8, Pull::Up);

    info!("Dummy started!");

    loop {
        if button.is_low() {
            led.set_high();
        } else {
            led.set_low();
        }

        Timer::after_millis(10).await;
    }
}
