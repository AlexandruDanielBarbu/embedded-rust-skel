#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_time::{Duration, Timer, with_timeout};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut red_led = Output::new(p.PA5, Level::Low, Speed::Medium);
    let mut yellow_led = Output::new(p.PA6, Level::Low, Speed::Medium);
    let mut green_led = Output::new(p.PA7, Level::Low, Speed::Medium);
    let switch = Input::new(p.PC9, Pull::Up);

    loop {
        // Init
        green_led.set_low();
        yellow_led.set_high();
        red_led.set_high();

        // logic
        if switch.is_low() {
            info!("S-a apasat butonul!!!");
            Timer::after_secs(2).await;

            green_led.set_high();
            yellow_led.set_low();

            Timer::after_secs(1).await;

            yellow_led.set_high();
            red_led.set_low();

            Timer::after_secs(3).await;
        } else {
            Timer::after_secs(1).await;
        }
    }
}
