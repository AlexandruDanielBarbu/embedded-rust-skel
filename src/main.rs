#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    crc::PolySize,
    gpio::{Input, Level, Output, OutputType, Pull, Speed},
    time::{hz, khz},
    timer::simple_pwm::{PwmPin, SimplePwm},
};
use embassy_time::Timer;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let servo_pin = PwmPin::new(p.PA0, OutputType::PushPull);

    let mut pwm = SimplePwm::new(
        p.TIM2,
        Some(servo_pin),
        None,
        None,
        None,
        hz(50),
        Default::default(),
    );

    info!("PWM started for servo!");

    let mut ch1 = pwm.ch1();
    ch1.enable();

    const MIN_PERIOD_US: u32 = 500;
    const MAX_PERIOD_US: u32 = 2500;
    const PERIOD_US: u32 = 20000;

    let min_value = (MIN_PERIOD_US * 1000) / PERIOD_US;
    let max_value = (MAX_PERIOD_US * 1000) / PERIOD_US;

    let WAIT = 100;
    loop {
        for pos in min_value..=max_value {
            ch1.set_duty_cycle_fraction(pos as u16, 1000);

            Timer::after_millis(WAIT).await;
        }

        for pos in (min_value..=max_value).rev() {
            ch1.set_duty_cycle_fraction(pos as u16, 1000);

            Timer::after_millis(WAIT).await;
        }

        Timer::after_millis(WAIT).await;
    }
}
