#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub enum MouseButton {
    // TODO: double clicks.
    Left,
    Right,
    Middle,
    Side,  // XBUTTON1 on win
    Extra, // XBUTTON2 on win
    #[cfg(target_os = "linux")]
    Forward,
    #[cfg(target_os = "linux")]
    Back,
    #[cfg(target_os = "linux")]
    Task,
}
