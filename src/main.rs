#![allow(unreachable_code)]

use std::error::Error;
use std::time::Duration;

use rouille::router;

use doorbot::open_door;

const HOLD_DURATION: Duration = Duration::from_secs(5);

fn main() -> Result<(), Box<dyn Error>> {
    rouille::start_server("0.0.0.0:8000", move |request| {
        router!(request,
            (GET) (/) => {
                open_door(HOLD_DURATION).expect("Umm...");
                rouille::Response::text("hello world")
            },
            _ => rouille::Response::empty_404()
        )
    });

    Ok(())
}
