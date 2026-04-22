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

    // ------------------------- EX 2

    // Setează adresa senzorului (schimbă în 0x76 dacă e nevoie)
    const BMP390_ADDR: u8 = 0x76;

    info!("Configurare BMP390...");

    // 1. Configurare OSR (0x1C): Supradeșantionare temperatură x2
    // Trimitem un array cu adresa registrului și valoarea dorită
    if let Err(_) = i2c.write(BMP390_ADDR, &[0x1C, 0b00_001_000]).await {
        error!("Eroare la scrierea in registrul OSR");
    }

    // 2. Configurare PWR_CTRL (0x1B): Mod Normal, Temp ON, Pres OFF
    if let Err(_) = i2c.write(BMP390_ADDR, &[0x1B, 0b00_11_00_10]).await {
        error!("Eroare la scrierea in registrul PWR_CTRL");
    }

    info!("Senzor configurat! Incepem citirea la fiecare secunda...");

    loop {
        // Vrem să citim 3 bytes începând cu DATA_3 (Adresa 0x07)
        let mut temp_data = [0u8; 3];

        // write_read trimite adresa registrului de start (0x07), apoi citește 3 bytes direct în temp_data
        match i2c.write_read(BMP390_ADDR, &[0x07], &mut temp_data).await {
            Ok(_) => {
                // temp_data[0] = DATA_3 (XLSB)
                // temp_data[1] = DATA_4 (LSB)
                // temp_data[2] = DATA_5 (MSB)

                // Asamblăm cele 3 bucăți de 8 biți într-un număr de 24 de biți (u32)
                let raw_temp: u32 = ((temp_data[2] as u32) << 16)
                    | ((temp_data[1] as u32) << 8)
                    | (temp_data[0] as u32);

                info!("Valoare Raw Temperatura: {}", raw_temp);
            }
            Err(e) => {
                // {:?} forteaza printarea detaliilor tehnice ale erorii (ex: Timeout, Nack, Overrun)
                error!("A esuat citirea! Motiv: {:?}", e);
            }
        }

        // Așteptăm 1 secundă
        Timer::after_secs(1).await;
    }
}
