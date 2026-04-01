#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::{Spawner, task};
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Instant, Timer};
use panic_probe as _;

fn busy_wait(ms: u64) {
    let start_time = Instant::now();
    while start_time.elapsed().as_millis() < ms {}
}

#[task(pool_size = 2)]
async fn led_blink(mut led_pin: Output<'static>) {
    loop {
        led_pin.set_low();
        // busy_wait(500);
        Timer::after_millis(500).await;

        led_pin.set_high();
        // busy_wait(500);
        Timer::after_millis(500).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_stm32::init(Default::default());
    info!("Device started Boyyyy");

    // The red LED is connected to D8 (PC7).
    let led_red = Output::new(peripherals.PC7, Level::High, Speed::Low);
    // The blue LED is connected to D9 (PC6).
    let led_blue = Output::new(peripherals.PC6, Level::High, Speed::Low);

    spawner.spawn(led_blink(led_red)).unwrap();
    spawner.spawn(led_blink(led_blue)).unwrap();
}
