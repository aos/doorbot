use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};
use rppal::system::DeviceInfo;
use rppal::pwm::{Channel, Polarity, Pwm};

const GPIO_LED: u8 = 23;

const PERIOD_MS: u64 = 20;
const PULSE_MIN_US: u64 = 800;
const PULSE_NEUTRAL_US: u64 = 1500;
const PULSE_MAX_US: u64 = 2300;
const HOLD_DURATION: Duration = Duration::from_secs(3);

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
    println!("Pulsed max 1");
    blink(&mut pin, HOLD_DURATION);

    pwm.set_pulse_width(Duration::from_micros(PULSE_MIN_US))?;
    println!("Pulsed min 2");
    blink(&mut pin, Duration::from_millis(500));

    Ok(())
}

fn blink(pin: &mut OutputPin, hold: Duration) {
    pin.set_high();
    thread::sleep(hold);
    pin.set_low();
}
