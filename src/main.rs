use mouse_keyboard_input::*;
use std::thread;
use std::time::Duration;

fn main() {
    use KeybdKey::*;
    bind_any_key(Action::handle(|key| {
        println!("Pressed: {:?}", key);
    }));
    A.bind(|_| {
        println!("AAAAAAnd we have a winner.");
    });
    for key in [T, H, I, S, Space, A, Space, T, E, S, T].iter() {
        key.click();
    }
    thread::sleep(Duration::from_secs(100));
}
