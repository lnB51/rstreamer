/* 
Lib file of rstreamer project.
License: MIT
Github repo: https://github.com/lnB51/rstreamer
*/


use std::{env, process};

pub struct Config {
    pub device_path: String,
    pub resolution: String,
    pub host: String,
    pub port: u16,
}

pub fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();

    // Default values
    let mut config = Config::default();

    // Parse command-line arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-d" | "--device" => {
                i += 1;
                if i < args.len() {
                    config.device_path = args[i].clone();
                } else {
                    println!("Error: Missing value for device argument.");
                    help();
                    return Config::default();
                }
            }
            "-r" | "--resolution" => {
                i += 1;
                if i < args.len() {
                    config.resolution = args[i].clone();
                } else {
                    println!("Error: Missing value for resolution argument.");
                    help();
                    return Config::default();
                }
            }
            "-h" | "--host" => {
                i += 1;
                if i < args.len() {
                    config.host = args[i].clone();
                } else {
                    println!("Error: Missing value for host argument.");
                    help();
                    return Config::default();
                }
            }
            "-p" | "--port" => {
                i += 1;
                if i < args.len() {
                    if let Ok(p) = args[i].parse::<u16>() {
                        config.port = p;
                    } else {
                        println!("Error: Invalid value for port argument.");
                        help();
                        return Config::default();
                    }
                } else {
                    println!("Error: Missing value for port argument.");
                    help();
                    return Config::default();
                }
            }
            "-hlp" | "--help" => {             
                help();                
                process::exit(0);
            }
            _ => {
                println!("Error: Unknown argument '{}'.", args[i]);
                help();
                process::exit(0);
            }
        }
        i += 1;
    }

    config
}

fn help() {
    println!("Usage:");
    println!("  -d, --device       Specify the video device (e.g., /dev/video0)");
    println!("  -r, --resolution   Specify the resolution (e.g., 1920x1080)");
    println!("  -h, --host         Specify the host address (e.g., 192.168.0.1)");
    println!("  -p, --port         Specify the port number (e.g., 80)");
}

impl Default for Config {
    fn default() -> Config {
        Config {
            device_path: "/dev/video0".to_string(),
            resolution: "1280x720".to_string(),
            host: "localhost".to_string(),
            port: 8080,
        }
    }
}
