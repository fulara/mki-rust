use mouse_keyboard_input::*;
use std::thread;
use std::time::Duration;

fn main() {
    use KeybdKey::*;
    bind_any_key(Action::handle_kb(|key| {
        println!("Pressed: {:?}", key);
    }));
    A.bind(|_| {
        println!("AAAAAAnd we have a winner.");
    });

    S.act_on(Action::sequencing_kb(|_| {
        println!("\nOkay mimicking a very slow operation... (1s sleep)");
        thread::sleep(Duration::from_millis(1000));
        println!("\nOkay action done (100ms sleep)");
        thread::sleep(Duration::from_millis(100));
    }));

    R.bind(|_| {
        println!("R Pressed sending Q");
        Q.click();
    });

    MouseButton::Left.bind(|_| {
        println!("Left Mouse button pressed");
    });

    MouseButton::Right.bind(|_| {
        println!("Right Mouse button pressed");
    });

    for key in [T, H, I, S, Space, A, Space, T, E, S, T].iter() {
        key.click();
    }
    thread::sleep(Duration::from_secs(100));
}
