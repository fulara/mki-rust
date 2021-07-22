use mki::{bind_key, Action, InhibitEvent, Keyboard, Sequence};
use std::thread;
use std::time::Duration;

fn main() {
    Keyboard::A.bind(|_| {
        println!("A pressed, sending B");
        Keyboard::B.click();
    });
    mki::bind_any_key(Action::handle_kb(|key| {
        use Keyboard::*;
        if matches!(key, S | L | O | W | LeftShift | LeftControl | B) {
            // Ignore outputs from other commands for nicer output
        } else {
            println!("Some key pressed pressed: {:?}", key);
        }
    }));
    mki::bind_any_button(Action::handle_mouse(|button| {
        println!("Mouse button pressed {:?}", button);
    }));
    mki::register_hotkey(&[Keyboard::LeftControl, Keyboard::B], || {
        println!("Ctrl+B Pressed")
    });
    mki::bind_key(
        Keyboard::S,
        Action::sequencing_kb(|_| {
            Sequence::text("LLLLLow").unwrap().send();
            thread::sleep(Duration::from_secs(1));
        }),
    );

    // This binds action to a W key,
    // that W press will not be sent to the following services ( only on windows )
    // whenever Caps Lock is toggled
    // Action will be executed on separate thread.
    bind_key(
        Keyboard::W,
        Action {
            callback: Box::new(|event, state| {
                println!("key: {:?} changed state now is: {:?}", event, state);
            }),
            inhibit: InhibitEvent::maybe(|| {
                if Keyboard::CapsLock.is_toggled() {
                    InhibitEvent::Yes
                } else {
                    InhibitEvent::No
                }
            }),
            sequencer: false,
            defer: true,
        },
    );

    thread::sleep(Duration::from_secs(100));
}
