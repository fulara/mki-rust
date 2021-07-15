use mouse_keyboard_input::*;
use std::thread;
use std::time::Duration;

fn main() {
    use KeybdKey::*;
    install_any_key_callback(|key| {
        println!("Pressed: {:?}", key);
        if key == KeybdKey::A {
            println!("And the winner is.");
        }
        InhibitEvent::No
    });
    for key in [T, H, I, S, Space, A, Space, T, E, S, T].iter() {
        key.click();
    }
    thread::sleep(Duration::from_secs(1));
}
