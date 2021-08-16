use std::io;

use rouille::router;
use rppal::system::DeviceInfo;

use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};
use rppal::pwm::{Channel, Polarity, Pwm};

const STATUS_LED: u8 = 23;
const OPEN_LED: u8 = 23;
const RESET_LED: u8 = 23;

const PERIOD_MS: u64 = 20;
const PULSE_MIN_US: u64 = 800;
const PULSE_MAX_US: u64 = 2300;

const DEFAULT_PORT: &str = "8000";
const HOLD_DURATION: Duration = Duration::from_secs(5);

pub struct LedPins {
    status: OutputPin,
    open: OutputPin,
    reset: OutputPin,
}

impl LedPins {
    fn new(status_num: u8, open_num: u8, reset_num: u8) -> Result<LedPins, Box<dyn Error>> {
        Ok(LedPins {
            status: Gpio::new()?.get(status_num)?.into_output(),
            open: Gpio::new()?.get(open_num)?.into_output(),
            reset: Gpio::new()?.get(reset_num)?.into_output(),
        })
    }

}

fn main() {
    println!("Working with {}", DeviceInfo::new().expect("device not found").model());
    let pwm = Pwm::with_period(
        Channel::Pwm0,
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MAX_US),
        Polarity::Normal,
        true,
    ).expect("set PWM");
    let led_pins = LedPins::new(STATUS_LED, OPEN_LED, RESET_LED).expect("led pins");


    let port = std::env::var("PORT").unwrap_or(DEFAULT_PORT.into());
    println!("Now listening on 0.0.0.0:{}", port);

    rouille::start_server(format!("0.0.0.0:{}", port), move |request| {
        rouille::log(&request, io::stdout(), || {
            router!(request,
                (GET) (/) => {
                    rouille::Response::html(INDEX)
                },
                (GET) (/sesame) => {
                    match pwm.set_pulse_width(Duration::from_micros(PULSE_MIN_US)) {
                        Ok(_) => rouille::Response::html(INDEX),
                        Err(e) => rouille::Response::html(format!("{}\n{}", INDEX, e)),
                    }
                },
                _ => rouille::Response::empty_404()
            )
        })
    });
}

pub fn open_door(pins: LedPins, hold: Duration) -> Result<(), Box<dyn Error>> {
    println!("Holding door open for... {:?}", hold);
    thread::sleep(hold);

    println!("Resetting...");

    Ok(())
}

fn blink(pin: &mut OutputPin, hold: Duration) {
    pin.set_high();
    thread::sleep(hold);
    pin.set_low();
}

const INDEX: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Open Sesame!</title>
    <style>
        body { min-height: 100vh; display: flex; flex-direction: column; justify-content: center; align-items: center; }
        #submit { box-shadow: inset 0 0 4px 0px #FFFFFF; background-color: #8E24AA; color: #ffffff; font-size: 2.5em; border: 1px solid #FFFFFF; border-radius: 6px; padding: 6px 24px; }
        #submit:disabled { background-color: gray; }
    </style>
</head>
<body>
    <form action="/sesame" method="GET" onsubmit="document.getElementById('submit').disabled=true">
        <input type="submit" id="submit" value="Open Sesame">
    </form>
</body>
</html>
"#;
