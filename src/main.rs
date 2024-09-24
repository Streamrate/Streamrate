extern crate core;

use std::{env, fs};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use anyhow::Result;
use dotenv::dotenv;
use obws::Client;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let password: Option<String>;
    
    match env::var("OBS_WEBSOCKET_PASSWORD") {
        Ok(value) => {
            if value == "" || value == "YOUR_PASSWORD_HERE" {
                println!("ERR: OBS_WEBSOCKET_PASSWORD is not defined in .env file");
                exit(1);
            }
            password = Some(value);
        }
        Err(_) => {
            println!("ERR: OBS_WEBSOCKET_PASSWORD is not dYOUR_PASSWORD_HEREefined in .env file");
            exit(1);
        }
    }

    match fs::exists("./obs_bitrate.txt") {
        Ok(_) => {
            match fs::write("./obs_bitrate.txt", "Connecting") {
                Ok(_) => {},
                Err(err) => {
                    println!("ERR: Unable to overwrite the obs_bitrate.txt file, {}", err);
                    println!("Resuming the process ...");
                }
            }
        },
        Err(_) => {
            match fs::write("./obs_bitrate.txt", "Connecting") {
                Ok(_) => {
                    println!("Created obs_bitrate.txt file");
                },
                Err(err) => {
                    println!("ERR: Unable to create obs_bitrate.txt file, ${}", err);
                    exit(1);
                },
            }
        }
    }

    println!("Connecting to OBS Websocket interface ...");

    let client = Client::connect("localhost", 4455, password).await?;

    println!("Successfully connected to OBS Websocket interface");

    let mut initialized = false;
    let mut last_output_bytes: u64 = 0;
    let mut last_output_duration: Duration = Duration::new(0, 0);

    loop {
        let stream_status = client.streaming().status().await?;
        if stream_status.active {
            if initialized {
                if (stream_status.bytes > 0) {
                    let bytes_delta = stream_status.bytes - last_output_bytes;
                    let duration_delta = stream_status.duration - last_output_duration;
                    let _bd = (bytes_delta * 8) as f64;
                    let _bitrate = _bd / duration_delta.as_seconds_f64() / 1000f64;
                    let bitrate = _bitrate - _bitrate % 1f64;
                    let file_content = format!("{} kbps", bitrate);
                    match fs::write("./obs_bitrate.txt", file_content) {
                        Ok(_) => {},
                        Err(err) => {
                            println!("ERR: Unable to overwrite the obs_bitrate.txt file, {}", err);
                            println!("Resuming the process ...");
                        }
                    }
                    println!("bitrate: {} kbps", bitrate);
                }
            }
            initialized = true;
            last_output_bytes = stream_status.bytes;
            last_output_duration = Duration::try_from(stream_status.duration)?;
            // println!("{stream_status:#?}");
        } else {
            match fs::write("./obs_bitrate.txt", "stream is not active") {
                Ok(_) => {},
                Err(_) => {},
            }
            println!("stream is not active");
        }
        sleep(Duration::from_millis(1000))
    }

    Ok(())
}

