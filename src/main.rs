use std::error::Error;
use std::time::Duration;

use doorbot::open_door;

const HOLD_DURATION: Duration = Duration::from_secs(3);

fn main() -> Result<(), Box<dyn Error>> {
    open_door(HOLD_DURATION)?;

    Ok(())
}
