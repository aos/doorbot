## Doorbot

This is a simple bot that presses a button on my apartment intercom to open the
building door.

### Requirements

- Raspberry Pi (currently using Zero W)
- Servo motor (currently using the `Tower Pro SG92R`)
- Rust `stable`
    - `rustup target add arm-unknown-linux-gnueabihf` (for Pi 3B)
    - `rustup target add arm-unknown-linux-gnueabi` (for Pi Zero W)

### Installation and Usage

1. Connect the servo leads to the RPi GPIO:
    - Brown = Ground
    - Red = 5V
    - Yellow = Output (`GPIO18`, the PWM pin)
2. Build application, see [Remote execution](#remote-execution)
3. Start web app:
    ```
    $ ./doorbot
    ```
    - Note: Make sure user is in `gpio` group to access `/dev/mem`, otherwise
      use `sudo`
4. Point browser to IP address of the Pi. Press button to operate motor.

### Remote execution

I am cross-compiling this application and `scp`ing to a remote Pi, see commands
below:
```
cargo build --target=arm-unknown-linux-gnueabihf
scp target/arm-unknown-linux-gnueabihf/debug/doorbot pi@192.168.20.254:
ssh -t pi@192.168.20.254 ./doorbot
```
