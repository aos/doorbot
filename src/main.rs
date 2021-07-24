use std::time::Duration;
use std::io;

use rouille::router;
use rppal::system::DeviceInfo;

use doorbot::open_door;

const DEFAULT_PORT: &str = "8000";
const HOLD_DURATION: Duration = Duration::from_secs(5);

fn main() {
    println!("Working with {}", DeviceInfo::new().expect("device not found").model());

    let port = std::env::var("PORT").unwrap_or(DEFAULT_PORT.into());
    println!("Now listening on 0.0.0.0:{}", port);

    rouille::start_server(format!("0.0.0.0:{}", port), move |request| {
        rouille::log(&request, io::stdout(), || {
            router!(request,
                (GET) (/) => {
                    rouille::Response::html(INDEX)
                },
                (GET) (/sesame) => {
                    match open_door(HOLD_DURATION) {
                        Ok(_) => rouille::Response::html(INDEX),
                        Err(e) => rouille::Response::html(format!("{}\n{}", INDEX, e)),
                    }
                },
                _ => rouille::Response::empty_404()
            )
        })
    });
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
