#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub(crate) mod details;

use crate::details::lock_registry;
#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(target_os = "windows")]
pub use windows::*;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
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

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum KeybdKey {
    Backspace,
    Tab,
    Enter,
    Escape,
    Space,
    Home,
    Left,
    Up,
    Right,
    Down,
    Insert,
    Delete,
    Numrow0,
    Numrow1,
    Numrow2,
    Numrow3,
    Numrow4,
    Numrow5,
    Numrow6,
    Numrow7,
    Numrow8,
    Numrow9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    NumLock,
    ScrollLock,
    CapsLock,
    LShift,
    RShift,
    LControl,
    RControl,
    Other(i32),
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum InhibitEvent {
    Yes,
    No,
}

pub fn install_any_key_callback(
    callback: impl Fn(KeybdKey) -> InhibitEvent + Send + Sync + 'static,
) {
    lock_registry().any_key_callback = Box::new(callback);
}

pub fn remove_any_key_callback() {
    lock_registry().any_key_callback = Box::new(|_| InhibitEvent::No);
}

pub fn install_key_callback(
    key: KeybdKey,
    callback: impl Fn(KeybdKey) -> InhibitEvent + Send + Sync + 'static,
) {
    lock_registry()
        .key_callbacks
        .insert(key, Box::new(callback));
}

pub fn remove_key_callback(key: KeybdKey) {
    lock_registry().key_callbacks.remove(&key);
}
