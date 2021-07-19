use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};
use rppal::system::DeviceInfo;
use rppal::pwm::{Channel, Polarity, Pwm};

const GPIO_LED: u8 = 23;

const PERIOD_MS: u64 = 20;
const PULSE_MIN_US: u64 = 1200;
const PULSE_NEUTRAL_US: u64 = 1500;
const PULSE_MAX_US: u64 = 1800;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Working with {}", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();
    let pwm = Pwm::with_period(
        Channel::Pwm0,
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MAX_US),
        Polarity::Normal,
        true,
    )?;

    thread::sleep(Duration::from_millis(500));

    blink(&mut pin);
    pwm.set_pulse_width(Duration::from_micros(PULSE_MIN_US))?;

    thread::sleep(Duration::from_millis(500));


    for pulse in (PULSE_MIN_US..=PULSE_NEUTRAL_US).step_by(10) {
        pwm.set_pulse_width(Duration::from_micros(pulse))?;
        thread::sleep(Duration::from_millis(20));
        blink(&mut pin);
    }

    Ok(())
}

fn blink(pin: &mut OutputPin) {
    pin.set_high();
    thread::sleep(Duration::from_millis(100));
    pin.set_low();
}
