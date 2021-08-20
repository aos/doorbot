use std::io;

use rouille::router;
use rppal::system::DeviceInfo;

use std::error::Error;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};
use rppal::pwm::{Channel, Polarity, Pwm};

const STATUS_LED: u8 = 17;
const OPEN_LED: u8 = 23;
const ERR_LED: u8 = 4;

const PERIOD_MS: u64 = 20;
const PULSE_MIN_US: u64 = 800;
const PULSE_MAX_US: u64 = 2300;

const DEFAULT_PORT: &str = "8000";
const HOLD_DURATION: Duration = Duration::from_secs(5);

pub struct LedPins {
    pub status: OutputPin,
    pub open: OutputPin,
    pub err: OutputPin,
}

impl LedPins {
    fn new(status_num: u8, open_num: u8, err_num: u8) -> Result<Mutex<LedPins>, Box<dyn Error>> {
        Ok(Mutex::new(LedPins {
            status: Gpio::new()?.get(status_num)?.into_output(),
            open: Gpio::new()?.get(open_num)?.into_output(),
            err: Gpio::new()?.get(err_num)?.into_output(),
        }))
    }
}

fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| DEFAULT_PORT.into());
    println!(
        "Working with {}",
        DeviceInfo::new().expect("device not found").model()
    );
    println!("Now listening on 0.0.0.0:{}", port);

    let led_pins = LedPins::new(STATUS_LED, OPEN_LED, ERR_LED).expect("led pins");
    led_pins.lock().unwrap().status.set_high();

    rouille::start_server(format!("0.0.0.0:{}", port), move |request| {
        rouille::log(&request, io::stdout(), || {
            router!(request,
                (GET) (/) => {
                    if !request.raw_query_string().is_empty() {
                        rouille::Response::html(
                            format!("{}\n\n{}", INDEX, "Something weird happened... try again.")
                        )
                    } else {
                        rouille::Response::html(INDEX)
                    }
                },
                (GET) (/sesame) => {
                    led_pins.lock().unwrap().status.set_low();
                    match open_door(&led_pins, HOLD_DURATION) {
                        Ok(_) => {
                            led_pins.lock().unwrap().err.set_low();
                            led_pins.lock().unwrap().status.set_high();
                            rouille::Response::redirect_303("/")
                        },
                        Err(e) => {
                            println!("Got error: {}", e);

                            led_pins.lock().unwrap().err.set_high();
                            rouille::Response::redirect_303("/?e=true")
                        }
                    }
                },
                _ => rouille::Response::empty_404()
            )
        })
    });
}

pub fn open_door(pins: &Mutex<LedPins>, hold: Duration) -> Result<(), Box<dyn Error>> {
    let pwm = Pwm::with_period(
        Channel::Pwm0,
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MAX_US),
        Polarity::Normal,
        true,
    )?;

    println!("Holding door open for... {:?}", hold);
    pins.lock().unwrap().open.set_high();
    thread::sleep(hold);
    pins.lock().unwrap().open.set_low();

    println!("Resetting...");
    pwm.set_pulse_width(Duration::from_micros(PULSE_MIN_US))?;
    thread::sleep(Duration::from_millis(150));
    Ok(())
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
