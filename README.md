# mki - mouse-keyboard-input 
Windows & Linux library for registring global input hooks and simulating keyboard and mouse events.

## Features:
* Install a global key/mouse event handler without binding to individual keys.
* Install a per key/button event handlers.
* Bind action on key presses.
* Register hotkeys combinations such as CTRL+Q and have action invoked on them.

## Sample:
Check examples/readme.rs for the example. Can be run with `cargo run --example readme`.

```rust
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
    mki::bind_key(Keyboard::S,Action::sequencing_kb(|_| {
        Sequence::text("LLLLLow").unwrap().send();
        thread::sleep(Duration::from_secs(1));
    }));

    // This binds action to a W key,
    // that W press will not be sent to the following services ( only on windows )
    // whenever Left Shift is Pressed
    // Action will be executed on separate thread.
    bind_key(Keyboard::W, Action {
        callback: Box::new(|event, state| {
            println!("key: {:?} changed state now is: {:?}", event, state);
        }),
        inhibit: InhibitEvent::No,
        sequencer: false,
        defer: true,
    });

    thread::sleep(Duration::from_secs(100));
}
```

## Threading model
It is strongly advised to use a default `bind` which will spawn new thread for the bindings.  
There is an option to `sequence` the events which will cause them to be invoked one after another in a separate thread.  
An option to `callback` the event causes invocation of the detected thread.  

Nomenclature used:
* `handle` -> spawns a new thread.
* `sequence` -> enqueues given event in a single thread that handles all the events one after another.
* `callback` -> callbacks on the same thread as the key was detected on, recommended not to block that thread nor 
schedule other key presses as it may result in handler being silently deregistered.

## Linux
Note that running the app on Linux requires root.

### Linux dependencies:
*libxtst-dev*

### Linux caveats

Currently the linux implementation will sleep for 100ms upon first invocation of the library.  
Otherwise some initial key strokes are missed.

##### cross development linux -> windows
cross.

to cross compile windows on linux:
```rust
cargo install cross
cross check --target x86_64-pc-windows-gnu

```
# Support
If you want to show appreciation for the stuff this repo gave you - you can do so via https://www.buymeacoffee.com/fulara
