## Doorbot

This is a simple bot that presses a button on my apartment intercom to open the
building door. It is operated via a `bottle.py` web application.

### Requirements

- Raspberry Pi
- Servo motor (I'm currently using the `Tower Pro SG92R`)
- Python 3 libraries: `RPi.GPIO`, `bottle`

### Installation and Usage

1. Connect the servo leads to the RPi GPIO:
    - Brown = Ground
    - Red = 5V
    - Orange = Output (pin `11` using `BOARD` numbering scheme)
2. Start web app:
    ```
    $ sudo python3 bot.py
    ```
    - Note: sudo is required here to access the `/dev/mem` device.
3. Point browser to IP address of the Pi. Press button to operate motor.
