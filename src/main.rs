#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::{Input, Io, Level, Output, Pull},
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};

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
    led.set_low(); // to turn the led on: led.set_high()

    // 3. Create an instance of a pushbutton on gpio port io5
    // and set the button Pull::Up (Pull::Down means pressed)
    let button = Input::new(io.pins.gpio20, Pull::Up);

    let delay = Delay::new(&clocks);

    loop {
        // log::info!("Privet Mir!");
        led.toggle();
        // delay.delay(500.millis());
        if button.is_high() {
            led.set_low();
        } else {
            log::info!("Button clicked!");
            led.set_high();
        }
        delay.delay_millis(500);
    }
}
