#![no_std]
#![no_main]

use core::cell::RefCell;
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Input, Level, Output, Pull, Speed},
    rcc::{Pll, PllDiv, PllMul, PllPreDiv, PllSource, Sysclk, VoltageScale},
    spi::{Config, Spi},
    time::Hertz,
};
use embassy_time::{Delay, Timer};
use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::Text,
};
use embedded_hal_bus::spi::{ExclusiveDevice, RefCellDevice};
use mipidsi::interface::SpiInterface;
use mipidsi::models::ST7735s;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Configuratii and shi
    // --- CONFIGURARE CLOCK (Din informațiile tale) ---
    let mut config = embassy_stm32::Config::default();
    config.rcc.hsi = true;
    config.rcc.pll1 = Some(Pll {
        source: PllSource::HSI,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL10,
        divp: None,
        divq: None,
        divr: Some(PllDiv::DIV1),
    });
    config.rcc.sys = Sysclk::PLL1_R;
    config.rcc.voltage_range = VoltageScale::RANGE1;
    let p = embassy_stm32::init(config);

    // --- CONFIGURARE SPI ȘI PINI ---
    let mut spi_config = embassy_stm32::spi::Config::default();
    spi_config.frequency = Hertz(3_000_000); // 3MHz e sigur pentru început

    let spi_bus = Spi::new(
        p.SPI1,
        p.PA5,
        p.PA7,
        p.PA6,
        p.GPDMA1_CH0,
        p.GPDMA1_CH1,
        spi_config,
    );
    let spi_bus_shared = RefCell::new(spi_bus);

    // Pinii de control (Asigură-te că aceștia sunt cei cablați de tine!)
    let cs_lcd = Output::new(p.PC7, Level::High, Speed::VeryHigh);
    let dc_lcd = Output::new(p.PA8, Level::Low, Speed::VeryHigh);
    let rst_lcd = Output::new(p.PC9, Level::High, Speed::VeryHigh); // Verifică pinul de Reset!

    // --- INIȚIALIZARE ECRAN ---
    let spi_device = RefCellDevice::new(&spi_bus_shared, cs_lcd, Delay).unwrap();

    // Buffer-ul necesar pentru mipidsi
    let mut buffer = [0u8; 512];
    let di = SpiInterface::new(spi_device, dc_lcd, &mut buffer);

    let mut screen = mipidsi::Builder::new(ST7735s, di)
        .reset_pin(rst_lcd)
        .color_order(mipidsi::options::ColorOrder::Rgb)
        .init(&mut Delay)
        .unwrap();

    // Ștergem ecranul cu negru
    screen.clear(Rgb565::BLACK).unwrap();

    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);

    let style = PrimitiveStyleBuilder::new().fill_color(Rgb565::RED).build();
    loop {
        // // Aici vei citi de la BMP390
        // let temp = 25; // Exemplu până legi driverul de senzor

        // // Afișăm pe ecran
        // screen.clear(Rgb565::BLACK).unwrap();
        // Text::new("Statie Meteo", Point::new(10, 20), style)
        //     .draw(&mut screen)
        //     .unwrap();

        // Text::new(format!("Temp: {} C", temp).as_str(), Point::new(10, 40), style)
        //     .draw(&mut screen)
        //     .unwrap();

        Rectangle::new(Point::new(0, 0), Size::new(50, 50))
            .into_styled(style)
            .draw(&mut screen)
            .unwrap();
        Timer::after_secs(1).await;
    }
}
