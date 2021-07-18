use mki::*;
use std::fs::File;
use std::io::Read;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), serde_yaml::Error> {
    let cfg = std::env::args()
        .nth(1)
        .expect("Expects 1 argument - path to config file");
    let mut file = File::open(cfg).expect("Failed to open cfg file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file contents to string");
    load_config(&content)?;
    thread::sleep(Duration::from_secs(u64::MAX));
    Ok(())
}
