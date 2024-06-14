#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::{Input, Io, Level, Output, Pull},
    i2c::I2C,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};
use sht31::{mode::Sht31Reader, Accuracy, TemperatureUnit, SHT31};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();

    esp_println::logger::init_logger_from_env();

    // 1. create an io instance
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    // 2. Create an instance of a led on a gpio port
    let mut led = Output::new(io.pins.gpio7, Level::Low);
    led.set_low();

    // 3. Create an instance of a pushbutton on gpio port io5
    // and set the button high (low means pressed)
    let button = Input::new(io.pins.gpio20, Pull::Up);

    let delay = Delay::new(&clocks);

    // 4. Get data from the SHT31 sensor I2C
    // SCL gpio37 SDL clock
    // SDA gpio38 SDA data
    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio38,
        io.pins.gpio37,
        100.kHz(),
        &clocks,
        None,
    );
    let mut sht = SHT31::new(i2c, delay)
        .with_accuracy(Accuracy::High)
        .with_unit(TemperatureUnit::Celsius);

    // 5. Instantiate the oled ssd1315
    // create an i2c instance
    // let (scl, sda) = (io.pins.gpio37, io.pins.gpio38);
    //

    loop {
        // log::info!("Privet Mir!");
        // led.toggle();
        // delay.delay(500.millis());
        if button.is_high() {
            led.set_low();
        } else {
            log::info!("Button clicked!");
            led.set_high();
        }

        // print out the values from the SHT32 sensor
        let reading = sht.read();
        log::info!("sht: {:?}", reading);

        // delay
        delay.delay(1000.millis());
    }
}
