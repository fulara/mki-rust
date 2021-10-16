use mki::*;
use std::env::args;
use std::fs::File;
use std::io::Read;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), serde_yaml::Error> {
    let cfg = args()
        .nth(1)
        .expect("Expects 1 argument - path to config file");
    let maybe_debug: Option<String> = args().nth(2);
    let mut file = File::open(cfg).expect("Failed to open cfg file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file contents to string");
    if let Some(maybe_debug) = maybe_debug {
        if maybe_debug == "--debug" {
            println!("Enabling debug.");
            mki::enable_debug();
        } else {
            println!("Unknown option passed in: {}, exiting", maybe_debug)
        }
    }
    load_config(&content)?;
    thread::sleep(Duration::from_secs(u64::MAX));
    Ok(())
}
