#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}
