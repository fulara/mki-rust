# mki - mouse-keyboard-input [![crates.io version](https://img.shields.io/crates/v/mki.svg)](https://crates.io/crates/mki)
Windows & Linux library for registring global input hooks and simulating keyboard and mouse events.

## Features:
* Install a global key/mouse event handler without binding to individual keys.
* Install a per key/button event handlers.
* Bind action on key presses.
* Register hotkeys combinations such as CTRL+Q and have action invoked on them.

## Sample:
Check examples/readme.rs for the example. Can be run with `cargo run --example readme`.

```rust
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
```
## Sample2: config file
Library supports loading hotkey scripting using a yaml configuration file. check `using_config.rs`
example to see more elaborate example. Below is a minimal one that speaks for itself.
```
---
bind:
  - description: Whenever Ctrl+L is clicked click K as well
    key:
      - LeftControl
      - L
    action:
      click:
        key:
          - K
```

a Library provides a binary `mki` that can be used to load the script.

## mki binary
Library providers a mki binary that takes 1 argument a path to a .yaml file,
the File will be parsed, it will print kind of hotkeys it registers and listen infinitely for the input.

It is really a mini hotkey application.

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
#### TODOs:
* Should `are_pressed` support Mouse? for now the `Pressed` in config ignores mouse.
* Get Mouse Position on linux and windows missing.

##### Future Eventual Considerations
* Ditch those static states that initialize god knows when, instead introduce a `Context`.
  However the callbacks from libraries will still require a global accessor, but it will defintiely be better.

#### 0.2 release will have:
* Mouse to support location.
* Mouse to support live tracking callback
* Mouse to support sending key strokes at given coordinates.
* Should sequenced be removed and instead a way using `parking_lot::channel` be introduced?
That would be more elastic in terms of usage. Slight problem that there would have to be a (tx,rx) pair held.
* Linux display usage is ultra ugly right now, just change it to a lambda.

# Support
If you want to show appreciation for the stuff this repo gave you - you can do so via https://www.buymeacoffee.com/fulara
