#![no_std]
#![no_main]

use core::cell::{Cell, RefCell};
use critical_section::Mutex;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Text, TextStyle},
};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::{Event, Gpio0, Gpio1, Input, PullUp, IO},
    i2c::I2C,
    interrupt,
    peripherals::{Interrupt, Peripherals},
    prelude::*,
};
use esp_println::println;
// use log::info;
use nstr::ToString;
use sht31::{prelude::*, Accuracy, TemperatureUnit, SHT31};
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

static BUTTON_MIN: Mutex<RefCell<Option<Gpio0<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));
static BUTTON_MAX: Mutex<RefCell<Option<Gpio1<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));
// static TEMPERATURE: Mutex<Cell<f32>> = Mutex::new(Cell::new(14.00_f32));

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    esp_println::logger::init_logger_from_env();

    // 1. create an io instance
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // 2. Create an instance of a led on a gpio port
    let mut led = io.pins.gpio7.into_push_pull_output();
    let _ = led.set_low();

    // let delay = DelayMs::new(&clocks);
    let delay = Delay::new(&clocks);

    // buttons for setting the temperature
    // buttons are input type
    // immutable since we will only be reading
    // let mut button_plus = io.pins.gpio1.into_pull_up_input();
    let mut button_minus = io.pins.gpio0.into_pull_up_input();

    button_minus.listen(Event::FallingEdge);
    interrupt::enable(Interrupt::GPIO, interrupt::Priority::Priority3).unwrap();
    critical_section::with(|cs| BUTTON_MIN.borrow_ref_mut(cs).replace(button_minus));

    // 4. Get data from the SHT31 sensor I2C
    // SCL gpio37 SDL clock
    // SDA gpio38 SDA data
    let i2c_sht = I2C::new(
        peripherals.I2C0,
        io.pins.gpio38,
        io.pins.gpio37,
        100_u32.kHz(),
        &clocks,
    );

    let mut sht = SHT31::new(i2c_sht, delay)
        .with_accuracy(Accuracy::High)
        .with_unit(TemperatureUnit::Celsius);

    // 5. Instantiate the oled ssd1306
    // create an i2c instance
    // sda gpio8 / sdc gpio9
    let sda = io.pins.gpio8;
    let scl = io.pins.gpio9;
    //
    let i2c_oled = I2C::new(peripherals.I2C1, sda, scl, 100_u32.kHz(), &clocks);
    let interface = I2CDisplayInterface::new(i2c_oled);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    Text::new("Hello Alex", Point::new(10, 10), text_style)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {
        // check if the buttons are pressed
        //
        // if button_plus.is_low().unwrap() {
        //     log::info!("button plus is pressed!");
        // }
        // if button_minus.is_low().unwrap() {
        //     log::info!("button minus is pressed!");
        // }

        // toggle the led every 2 secs and do a reading
        // led.toggle();
        // delay.delay_micros(5_000_000);
        //
        display.clear_buffer();
        display.flush().unwrap();
        let _ = led.set_high();
        // print out the values from the SHT32 sensor
        let reading = sht.read().unwrap();
        let temperature = reading.temperature.to_string::<10>();
        let humidity = reading.humidity.to_string::<10>();

        Text::new(&temperature, Point::new(10, 30), text_style)
            .draw(&mut display)
            .unwrap();

        Text::new(&humidity, Point::new(10, 50), text_style)
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();
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
        let _ = led.set_low();
        log::info!("end of the loop");
    }
}

#[esp_hal::prelude::interrupt]
fn GPIO() {
    // Interrupt Service Routine Code
    println!("Gpio interrupt launched!");
    // log::info!("gpio0 interrupt");
    // start critical_section
    critical_section::with(|cs| {
        BUTTON_MIN
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt();
    })
}
