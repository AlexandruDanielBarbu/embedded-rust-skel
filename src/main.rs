#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::{Spawner, task};
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Instant, Timer};
use panic_probe as _;

#[task(pool_size = 4)]
async fn led_blink(mut led_pin: Output<'static>, freq: u64) {
    let mils = 1000 / freq / 2; // for the light to be visibly on
    loop {
        led_pin.set_low();
        Timer::after_millis(mils).await;

        led_pin.set_high();
        Timer::after_millis(mils).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_stm32::init(Default::default());
    info!("S-a dat drumul! :)))))))");

    // The red LED is connected to D8 (PC7).
    let led_red = Output::new(peripherals.PC7, Level::High, Speed::Low);
    // The blue LED is connected to D9 (PC6).
    let led_blue = Output::new(peripherals.PC6, Level::High, Speed::Low);
    // The green LED is connected to D11 (PA7).
    let led_green = Output::new(peripherals.PA7, Level::High, Speed::Low);
    // The yellow LED is connected to D10 (PC9).
    let led_yellow = Output::new(peripherals.PC9, Level::High, Speed::Low);

    spawner.spawn(led_blink(led_red, 4)).unwrap();
    spawner.spawn(led_blink(led_blue, 1)).unwrap();
    spawner.spawn(led_blink(led_green, 5)).unwrap();
    spawner.spawn(led_blink(led_yellow, 3)).unwrap();

    // spawner.spawn(led_blink(led_red, 4)).unwrap();
    // spawner.spawn(led_blink(led_blue, 4)).unwrap();
    // spawner.spawn(led_blink(led_green, 4)).unwrap();
    // spawner.spawn(led_blink(led_yellow, 4)).unwrap();
}
