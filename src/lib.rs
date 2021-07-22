pub(crate) mod details;

mod keyboard;
#[cfg(target_os = "linux")]
mod linux;
mod mouse;
mod parse;
mod sequence;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
use crate::linux::keyboard_mouse::kimpl;
#[cfg(target_os = "linux")]
use crate::linux::keyboard_mouse::mimpl;
#[cfg(target_os = "windows")]
use crate::windows::keyboard::kimpl;
#[cfg(target_os = "windows")]
use crate::windows::mouse::mimpl;
pub use keyboard::*;
#[cfg(target_os = "linux")]
pub use linux::*;
pub use mouse::*;
pub use parse::load_config;
pub use sequence::Sequence;
#[cfg(target_os = "windows")]
pub use windows::*;

use crate::details::registry;
use std::sync::Arc;

#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
/// Whether given button is now Pressed or Released.
/// Send in some version of the callbacks.
pub enum State {
    Pressed,
    Released,
}

impl Keyboard {
    /// Send an event to Press this key
    pub fn press(&self) {
        kimpl::press(*self)
    }

    /// Send an event to Release this key
    pub fn release(&self) {
        kimpl::release(*self)
    }
    /// Send an event to Click (Press + Release) this key
    pub fn click(&self) {
        kimpl::click(*self);
    }

    // Some buttons are toggleable like caps lock.
    /// Whether this KeyboardKey is toggled, applies for some buttons such as Caps Lock
    pub fn is_toggled(&self) -> bool {
        kimpl::is_toggled(*self)
    }

    /// Bind an action on this KeyboardKey, action will be invoked on a new thread.
    pub fn bind(&self, handler: impl Fn(Keyboard) + Send + Sync + 'static) {
        bind_key(*self, Action::handle_kb(handler))
    }

    /// opposite to `bind`. Clears bind
    pub fn clear_bind(&self) {
        remove_key_bind(*self);
    }

    /// Binds an action on this KeyboardKey, a version of `bind` that can do more.
    pub fn act_on(&self, action: Action) {
        bind_key(*self, action)
    }

    /// Whether given KeyboardKey is pressed.
    pub fn is_pressed(&self) -> bool {
        registry().is_pressed(Event::Keyboard(*self))
    }
}

impl Mouse {
    /// Send an event to Press this Button
    fn press(&self) {
        mimpl::press(*self)
    }

    /// Send an event to Release this Button
    fn click(&self) {
        mimpl::click(*self)
    }

    /// Send an event to Click (Press + Release) this key
    fn release(&self) {
        mimpl::release(*self)
    }

    /// Bind an action on this MouseButton, action will be invoked on a new thread.
    pub fn bind(&self, handler: impl Fn(Mouse) + Send + Sync + 'static) {
        bind_button(*self, Action::handle_mouse(handler))
    }

    /// opposite to `bind`. Clears bind
    pub fn clear_bind(&self) {
        remove_button_bind(*self);
    }

    /// Binds an action on this MouseButton, a version of `bind` that can do more.
    pub fn act_on(&self, action: Action) {
        bind_button(*self, action)
    }

    /// Whether given MouseButton is pressed.
    pub fn is_pressed(&self) -> bool {
        registry().is_pressed(Event::Mouse(*self))
    }
}

#[cfg(target_os = "windows")]
pub fn move_mouse(x: i32, y: i32) {
    move_mouse_impl(x, y);
}

#[derive(Clone)]
/// Works only on windows.
/// Whether to propagate the event for applications down the callstack.
pub enum InhibitEvent {
    Yes,
    Maybe(Arc<Box<dyn Fn() -> InhibitEvent + Send + Sync>>),
    No,
}

impl InhibitEvent {
    #[cfg(target_os = "windows")]
    fn should_inhibit(&self) -> bool {
        match self {
            InhibitEvent::Yes => true,
            InhibitEvent::Maybe(f) => {
                matches!(f(), InhibitEvent::Yes)
            }
            InhibitEvent::No => false,
        }
    }

    pub fn maybe(f: impl Fn() -> InhibitEvent + Send + Sync + 'static) -> Self {
        InhibitEvent::Maybe(Arc::new(Box::new(f)))
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
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
    pub fn handle_kb(action: impl Fn(Keyboard) + Send + Sync + 'static) -> Self {
        Self::handle(move |event| {
            if let Event::Keyboard(key) = event {
                action(key);
            }
        })
    }

    /// Version of `handle_kb` but for mouse.
    pub fn handle_mouse(action: impl Fn(Mouse) + Send + Sync + 'static) -> Self {
        Self::handle(move |event| {
            if let Event::Mouse(button) = event {
                action(button);
            }
        })
    }

    /// General version of `handle_kb` for both Mouse and Keyboard.
    pub fn handle(action: impl Fn(Event) + Send + Sync + 'static) -> Self {
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
    pub fn callback_kb(action: impl Fn(Keyboard) + Send + Sync + 'static) -> Self {
        Self::callback(move |event| {
            if let Event::Keyboard(key) = event {
                action(key);
            }
        })
    }

    /// Version of `callback_kb` but for mouse.
    pub fn callback_mouse(action: impl Fn(Mouse) + Send + Sync + 'static) -> Self {
        Self::callback(move |event| {
            if let Event::Mouse(button) = event {
                action(button);
            }
        })
    }

    /// General version of `callback_kb` for both Mouse and Keyboard.
    pub fn callback(action: impl Fn(Event) + Send + Sync + 'static) -> Self {
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
    pub fn sequencing_kb(action: impl Fn(Keyboard) + Send + Sync + 'static) -> Self {
        Self::sequencing(move |event| {
            if let Event::Keyboard(key) = event {
                action(key);
            }
        })
    }

    /// Version of `sequencing_kb` but for mouse.
    pub fn sequencing_mouse(action: impl Fn(Mouse) + Send + Sync + 'static) -> Self {
        Self::sequencing(move |event| {
            if let Event::Mouse(button) = event {
                action(button);
            }
        })
    }

    /// General version of `sequencing_kb` for both Mouse and Keyboard.
    pub fn sequencing(action: impl Fn(Event) + Send + Sync + 'static) -> Self {
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

/// Install any key handler that will be invoked on any key presses.
/// ```
/// use mki::*;
///
/// fn install_global_handler() {
///   bind_any_key(Action::handle_kb(|(key)| println!("Some key pressed: {:?}", key)));
/// }
/// ```
pub fn bind_any_key(action: Action) {
    *registry().any_key_callback.lock().unwrap() = Some(Arc::new(action))
}

/// Install any key handler that will be invoked on specified key presses.
///```
/// use mki::*;
///
/// fn bind_some_key() {
///   bind_key(Keyboard::B, Action::handle_kb(|(key)| println!("B Pressed")));
/// }
/// ```
pub fn bind_key(key: Keyboard, action: Action) {
    registry()
        .key_callbacks
        .lock()
        .unwrap()
        .insert(key, Arc::new(action));
}

/// Removes global key handler.
///```
/// use mki::*;
///
/// fn remove_global_handler() {
///   bind_any_key(Action::handle_kb(|(key)| println!("Some key pressed: {:?}", key)));
///   remove_any_key_bind();
/// }
/// ```
pub fn remove_any_key_bind() {
    *registry().any_key_callback.lock().unwrap() = None;
}

/// Removes specific key bind.
pub fn remove_key_bind(key: Keyboard) {
    registry().key_callbacks.lock().unwrap().remove(&key);
}

/// Same as `bind_any_key` but for mouse buttons.
pub fn bind_any_button(action: Action) {
    *registry().any_button_callback.lock().unwrap() = Some(Arc::new(action))
}

/// Same as `bind_key` but for mouse buttons.
pub fn bind_button(button: Mouse, action: Action) {
    registry()
        .button_callbacks
        .lock()
        .unwrap()
        .insert(button, Arc::new(action));
}

/// Same as `remove_any_key_bind` but for mouse buttons.
pub fn remove_any_button_bind() {
    *registry().any_button_callback.lock().unwrap() = None;
}

/// Same as `remove_key_bind` but for mouse buttons.
pub fn remove_button_bind(button: Mouse) {
    registry().button_callbacks.lock().unwrap().remove(&button);
}

/// Allows for registering an action that will be triggered when sequence of buttons is pressed.
/// callback will be invoked whenever last key of the sequence is pressed.
/// ```
/// use mki::*;
///
/// fn register() {
///   register_hotkey(&[Keyboard::LeftControl, Keyboard::B], || println!("CTRL+B pressed"));
/// }
/// ```
pub fn register_hotkey(sequence: &[Keyboard], callback: impl Fn() + Send + Sync + 'static) {
    registry().register_hotkey(sequence, callback);
}

/// Returns whether given key sequence is currently pressed down, this may be a single key.
pub fn are_pressed(sequence: &[Keyboard]) -> bool {
    registry().are_pressed(sequence)
}

/// Allows storing some kind of state within the library,
/// Generally not that useful but allows for some more complicated logic using yaml load.
pub fn set_state(key: &str, value: &str) {
    registry().set_state(key, value)
}

/// Returns the state, it has to be set beforehand with the set otherwise will be returned empty.
pub fn get_state(key: &str) -> Option<String> {
    registry().get_state(key)
}

/// Unregisters hotkey, a original sequence has to be passed as parameter..
pub fn unregister_hotkey(sequence: &[Keyboard]) {
    registry().unregister_hotkey(sequence);
}

pub fn set_mouse_tracker(f: impl Fn(i32, i32) + Send + Sync + 'static) {
    registry().set_mouse_tracker(Some(Arc::new(Box::new(f))));
}
