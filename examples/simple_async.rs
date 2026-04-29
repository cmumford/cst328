#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use cst328::Cst328;
use embassy_executor::Spawner;
use embassy_time::{Delay, Timer};
use esp_hal::{
    clock::CpuClock,
    gpio::{Level, Output, OutputConfig},
    i2c::master::I2c,
    time::Rate,
};
use log::{LevelFilter, info};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[esp_rtos::main]
async fn main(_spawner: Spawner) -> ! {
    esp_println::logger::init_logger(LevelFilter::Info);

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    cfg_if::cfg_if! {
        if #[cfg(feature = "esp32c6")] {
            let sda_gpio = peripherals.GPIO15;
            let scl_gpio = peripherals.GPIO23;
            let rst_gpio = peripherals.GPIO11;
        } else {
            let sda_gpio = peripherals.GPIO9;
            let scl_gpio = peripherals.GPIO8;
            let rst_gpio = peripherals.GPIO11;
        }
    }

    info!("[main] Initializing I2C");
    let i2c = I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(100)),
    )
    .unwrap()
    .into_async()
    .with_scl(scl_gpio)
    .with_sda(sda_gpio);

    let mut dev = Cst328::new(i2c);

    let mut rst_pin = Output::new(rst_gpio, Level::High, OutputConfig::default());
    let mut delay = Delay {};
    match cst328::reset(&mut rst_pin, &mut delay) {
        Ok(()) => info!("Reset OK"),
        Err(e) => info!("Reset failed: {e:?}"),
    }

    match dev.ping().await {
        Ok(()) => info!("Ping OK"),
        Err(e) => info!("Ping failed: {e:?}"),
    }

    loop {
        info!("Waiting...");
        Timer::after(embassy_time::Duration::from_millis(1_000)).await;
    }
}
