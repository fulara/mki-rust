use mki::*;
use std::thread;
use std::time::Duration;

fn main() {
    use Keyboard::*;
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

    #[cfg(target_os = "windows")] // Not sure how to detect double on linux
    MouseButton::DoubleLeft.bind(|_| {
        println!("Double Left Click Mouse");
    });

    MouseButton::Right.bind(|_| {
        println!("Right Mouse button pressed");
    });

    O.bind(|_| loop {
        println!("Observing T: {}", T.is_pressed());
        thread::sleep(Duration::from_secs(1));
    });

    for key in [T, H, I, S, Space, A, Space, T, E, S, T].iter() {
        key.click();
    }

    register_hotkey(&[LeftControl, U], || println!("Ctrl+U pressed"));

    B.bind(|_| {
        // So this seems awfully laggy on Linux, interesting. Is something wrong with impl?
        Sequence::text("mini mini mini mini mini mini")
            .unwrap()
            .send();
    });
    thread::sleep(Duration::from_secs(100));
}
