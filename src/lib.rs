pub(crate) mod details;

mod buttons;
mod keys;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub use buttons::*;
pub use keys::*;
#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(target_os = "windows")]
pub use windows::*;

use crate::details::lock_registry;
use std::sync::Arc;

#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub enum KeyState {
    Pressed,
    Released,
}

// MouseButton implements.
pub trait Button {
    fn press(&self);
    // Sends a down + release event
    fn click(&self);
    fn release(&self);

    fn is_pressed(&self) -> bool;
}

// KeybdKey implements.
pub trait Key {
    fn press(&self);
    fn release(&self);
    fn click(&self);

    fn is_pressed(&self) -> bool;
    // Some buttons are toggleable like caps lock.
    fn is_toggled(&self) -> bool;
}

impl KeybdKey {
    pub fn bind(&self, handler: impl Fn(KeybdKey) + Clone + Send + Sync + 'static) {
        bind_key(*self, Action::handle(handler))
    }

    pub fn act(&self, action: Action) {
        bind_key(*self, action)
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum InhibitEvent {
    Yes,
    No,
}

pub struct Action {
    /// What do you want to do on the key callback, see `defer` and `sequencer` to understand
    /// on which thread those are invoked.
    pub callback: Box<dyn Fn(KeybdKey, KeyState) + Send + Sync + 'static>,
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
    pub fn handle(action: impl Fn(KeybdKey) + Clone + Send + Sync + 'static) -> Self {
        Action {
            callback: Box::new(move |key, state| {
                if state == KeyState::Pressed {
                    action(key)
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
    pub fn callback(action: impl Fn(KeybdKey) + Clone + Send + Sync + 'static) -> Self {
        Action {
            callback: Box::new(move |key, state| {
                if state == KeyState::Pressed {
                    action(key)
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
    pub fn sequencing(action: impl Fn(KeybdKey) + Clone + Send + Sync + 'static) -> Self {
        Action {
            callback: Box::new(move |key, state| {
                if state == KeyState::Pressed {
                    action(key)
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

pub fn bind_key(key: KeybdKey, action: Action) {
    lock_registry().key_callbacks.insert(key, Arc::new(action));
}

pub fn remove_any_key_bind() {
    lock_registry().any_key_callback = None;
}

pub fn remove_key_bind(key: KeybdKey) {
    lock_registry().key_callbacks.remove(&key);
}
