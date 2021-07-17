pub(crate) mod details;

mod buttons;
mod keys;
#[cfg(target_os = "linux")]
mod linux;
mod sequence;
#[cfg(target_os = "windows")]
mod windows;

pub use buttons::*;
pub use keys::*;
#[cfg(target_os = "linux")]
pub use linux::*;
pub use sequence::Sequence;
#[cfg(target_os = "windows")]
pub use windows::*;

use crate::details::lock_registry;
use std::sync::Arc;

#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub enum State {
    Pressed,
    Released,
}

// MouseButton implements.
pub trait Button {
    fn press(&self);
    // Sends a down + release event
    fn click(&self);
    fn release(&self);
}

// KeybdKey implements.
pub trait Key {
    fn press(&self);
    fn release(&self);
    fn click(&self);

    // Some buttons are toggleable like caps lock.
    fn is_toggled(&self) -> bool;
}

impl Keyboard {
    pub fn bind(&self, handler: impl Fn(Keyboard) + Clone + Send + Sync + 'static) {
        bind_key(*self, Action::handle_kb(handler))
    }

    pub fn act_on(&self, action: Action) {
        bind_key(*self, action)
    }

    pub fn is_pressed(&self) -> bool {
        lock_registry().is_pressed(Event::Keyboard(*self))
    }
}

impl Mouse {
    pub fn bind(&self, handler: impl Fn(Mouse) + Clone + Send + Sync + 'static) {
        bind_button(*self, Action::handle_mouse(handler))
    }

    pub fn act_on(&self, action: Action) {
        bind_button(*self, action)
    }

    pub fn is_pressed(&self) -> bool {
        lock_registry().is_pressed(Event::Mouse(*self))
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum InhibitEvent {
    Yes,
    No,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Event {
    Keyboard(Keyboard),
    Mouse(Mouse),
}

pub struct Action {
    /// What do you want to do on the key callback, see `defer` and `sequencer` to understand
    /// on which thread those are invoked.
    pub callback: Box<dyn Fn(Event, State) + Send + Sync + 'static>,
    /// Whether to inhibit the event propagation to further applications down the call stack.
    /// This only works on windows.
    /// Note that for now the 'release' event cannot be inhibited.
    pub inhibit: InhibitEvent,
    /// This is the recommended mode, to 'defer' this causes every callback to be spawned on a new thread.
    /// On windows you cannot inject a new events from the callback invoked on the same thread
    /// As that would result this application to be removed from the queue. hence deferring is recommended.
    pub defer: bool,
    /// Very similar to defer but the callbacks are all sequenced in one thread.
    /// This is helpful if you are want to have slow tasks that should not overlap with one another.
    pub sequencer: bool,
}

impl Action {
    /// Helper to create probably the most common key bind.
    /// handler will be spawned in new thread
    /// will only react to key press not a release.
    /// Use this if you want to send inputs from the handlers as on windows it is not allowed
    /// to pump new events.
    /// will not inhibit event.
    pub fn handle_kb(action: impl Fn(Keyboard) + Clone + Send + Sync + 'static) -> Self {
        Self::handle(move |event| {
            if let Event::Keyboard(key) = event {
                action(key);
            }
        })
    }

    pub fn handle_mouse(action: impl Fn(Mouse) + Clone + Send + Sync + 'static) -> Self {
        Self::handle(move |event| {
            if let Event::Mouse(button) = event {
                action(button);
            }
        })
    }

    pub fn handle(action: impl Fn(Event) + Clone + Send + Sync + 'static) -> Self {
        Action {
            callback: Box::new(move |event, state| {
                if state == State::Pressed {
                    action(event)
                }
            }),
            inhibit: InhibitEvent::No,
            defer: true,
            sequencer: false,
        }
    }

    /// Helper to create callback.
    /// will only react to key press not a release.
    /// will not inhibit event.
    /// Use this if you want a simple handler without spawning threads.
    pub fn callback_kb(action: impl Fn(Keyboard) + Clone + Send + Sync + 'static) -> Self {
        Self::callback(move |event| {
            if let Event::Keyboard(key) = event {
                action(key);
            }
        })
    }

    pub fn callback_mouse(action: impl Fn(Mouse) + Clone + Send + Sync + 'static) -> Self {
        Self::callback(move |event| {
            if let Event::Mouse(button) = event {
                action(button);
            }
        })
    }

    pub fn callback(action: impl Fn(Event) + Clone + Send + Sync + 'static) -> Self {
        Action {
            callback: Box::new(move |event, state| {
                if state == State::Pressed {
                    action(event)
                }
            }),
            inhibit: InhibitEvent::No,
            defer: false,
            sequencer: false,
        }
    }

    /// Helper to create sequencing handler.
    /// Handler will be executed one after another in a dedicated thread
    /// will only react to key press not a release.
    /// will not inhibit event.
    /// Use this if you want to have complicated actions that do not overlap.
    pub fn sequencing_kb(action: impl Fn(Keyboard) + Clone + Send + Sync + 'static) -> Self {
        Self::sequencing(move |event| {
            if let Event::Keyboard(key) = event {
                action(key);
            }
        })
    }

    pub fn sequencing_mouse(action: impl Fn(Mouse) + Clone + Send + Sync + 'static) -> Self {
        Self::sequencing(move |event| {
            if let Event::Mouse(button) = event {
                action(button);
            }
        })
    }

    pub fn sequencing(action: impl Fn(Event) + Clone + Send + Sync + 'static) -> Self {
        Action {
            callback: Box::new(move |event, state| {
                if state == State::Pressed {
                    action(event)
                }
            }),
            inhibit: InhibitEvent::No,
            defer: false,
            sequencer: true,
        }
    }
}

pub fn bind_any_key(action: Action) {
    lock_registry().any_key_callback = Some(Arc::new(action))
}

pub fn bind_key(key: Keyboard, action: Action) {
    lock_registry().key_callbacks.insert(key, Arc::new(action));
}

pub fn remove_any_key_bind() {
    lock_registry().any_key_callback = None;
}

pub fn remove_key_bind(key: Keyboard) {
    lock_registry().key_callbacks.remove(&key);
}

pub fn bind_any_button(action: Action) {
    lock_registry().any_button_callback = Some(Arc::new(action))
}

pub fn bind_button(button: Mouse, action: Action) {
    lock_registry()
        .button_callbacks
        .insert(button, Arc::new(action));
}

pub fn remove_any_button_bind() {
    lock_registry().any_button_callback = None;
}

pub fn remove_button_bind(button: Mouse) {
    lock_registry().button_callbacks.remove(&button);
}

pub fn register_hotkey(sequence: &[Keyboard], callback: impl Fn() + Clone + Send + Sync + 'static) {
    lock_registry().register_hotkey(sequence, callback);
}

pub fn unregister_hotkey(sequence: &[Keyboard]) {
    lock_registry().unregister_hotkey(sequence);
}
