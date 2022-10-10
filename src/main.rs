/**
 * Minimal example to enable an aircon with the Daikin BRP072A4 Wi-Fi adapter.
 *
 * Usage:
 * ./aircon HOST TEMPERATURE
 *
 * Examples:
 * ./aircon 192.168.0.3 27
 * ./aircon 192.168.0.3 off
 *
 * Set TEMPERATURE to any non-number to switch off
 */
use std::{env, process};
use ureq::Response;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    host: String,
    temp: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments. Usage: \"ac HOST TEMP\", e.g. \"ac 192.168.0.3 27\"");
        }
        let host = args[1].clone();
        let temp = args[2].clone();
        Ok(Config { host, temp })
    }
}

// Main "logic" of this program. Just calling a URL.
fn run(config: Config) -> Result<Response, ureq::Error> {
    let (pow, temp) = match config.temp.parse::<f32>() {
        Ok(n) => (1, n),
        Err(_) => (0, 20.)
    };
    let path = format!("http://{host}/aircon/set_control_info?pow={pow}&mode=3&stemp={temp}&shum=0", host=config.host);
    ureq::get(&path).call()
}
