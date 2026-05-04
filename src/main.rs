#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Input, Level, Output, OutputType, Pull, Speed},
    time::khz,
    timer::simple_pwm::{PwmPin, SimplePwm},
};
use embassy_time::Timer;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut led_pin = PwmPin::new(p.PA0, OutputType::PushPull);

    let mut pwm = SimplePwm::new(
        p.TIM2,
        Some(led_pin),
        None,
        None,
        None,
        khz(1),
        Default::default(),
    );

    info!("PWM started!");

    let mut ch1 = pwm.ch1();
    ch1.enable();

    loop {
        // ch1.set_duty_cycle_percent(50);
        ch1.set_duty_cycle_percent(10);
        // ch1.set_duty_cycle_percent(100);

        Timer::after_millis(10).await;
    }
}
