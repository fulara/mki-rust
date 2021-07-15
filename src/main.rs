use mouse_keyboard_input::*;
use std::thread;
use std::time::Duration;

fn main() {
    use KeybdKey::*;
    for key in [T, H, I, S, Space, A, Space, T, E, S, T].iter() {
        key.click();
    }
    thread::sleep(Duration::from_secs(1));
}
