#![no_std]
#![no_main]

use embedded_graphics::{
    mono_font::{iso_8859_1::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Text, TextStyle},
};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::IO,
    // i2c::I2C,
    peripherals::Peripherals,
    prelude::_fugit_ExtU32,
    prelude::*,
    spi::{master::Spi, SpiMode},
};
// use nstr::ToString;
// use sht31::{prelude::*, Accuracy, TemperatureUnit, SHT31};
use ssd1306::{mode::BufferedGraphicsMode, prelude::SPIInterfaceNoCS, prelude::*, Ssd1306};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    esp_println::logger::init_logger_from_env();

    // 1. create an io instance
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let sclk = io.pins.gpio12;
    let miso = io.pins.gpio13;
    let mosi = io.pins.gpio11;
    let cs = io.pins.gpio10;
    let dc = io.pins.gpio3.into_push_pull_output();

    let spi = Spi::new(peripherals.SPI2, 100u32.kHz(), SpiMode::Mode0, &clocks).with_pins(
        Some(sclk),
        Some(mosi),
        Some(miso),
        Some(cs),
    );

    let interface = SPIInterfaceNoCS::new(spi, dc);

    // 2. Create an instance of a led on a gpio port
    // let mut led = io.pins.gpio7.into_push_pull_output();
    // let _ = led.set_low();

    // let delay = DelayMs::new(&clocks);
    let delay = Delay::new(&clocks);

    //
    // 4. Get data from the SHT31 sensor I2C
    // SCL gpio37 SDL clock
    // SDA gpio38 SDA data
    // let i2c_sht = I2C::new(
    //     peripherals.I2C0,
    //     io.pins.gpio38,
    //     io.pins.gpio37,
    //     100_u32.kHz(),
    //     &clocks,
    // );
    //
    // let mut sht = SHT31::new(i2c_sht, delay)
    //     .with_accuracy(Accuracy::High)
    //     .with_unit(TemperatureUnit::Celsius);

    // 5. Instantiate the oled ssd1306
    // create an i2c instance
    // sda gpio8 / sdc gpio9
    // let sda = io.pins.gpio8;
    // let scl = io.pins.gpio9;
    // //
    // let i2c_oled = I2C::new(peripherals.I2C1, sda, scl, 100_u32.kHz(), &clocks);
    // let interface = I2CDisplayInterface::new(i2c_oled);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();
    //
    let text_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);

    Text::new("Starting measurements ... ", Point::new(10, 10), text_style)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {
        // toggle the led every 2 secs and do a reading
        // led.toggle();
        delay.delay_micros(5_000_000);
        display.clear_buffer();
        display.flush().unwrap();
        // let _ = led.set_high();
        // print out the values from the SHT32 sensor
        // let reading = sht.read().unwrap();
        // let temperature = reading.temperature.to_string::<10>();
        // let humidity = reading.humidity.to_string::<10>();
        //
        // Text::new(&temperature, Point::new(10, 30), text_style)
        //     .draw(&mut display)
        //     .unwrap();
        //
        // Text::new(&humidity, Point::new(10, 50), text_style)
        //     .draw(&mut display)
        //     .unwrap();
        //
        // display.flush().unwrap();
        // match reading {
        //     Ok(value) => log::info!(
        //         "temperature: {} humidity: {}",
        //         value.temperature,
        //         value.humidity
        //     ),
        //     Err(_) => log::info!("no value"),
        // }
        delay.delay_micros(5_000_000);
        // Write to led
        // let _ = led.set_low();
    }
}
