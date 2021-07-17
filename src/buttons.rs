#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Side,
    Extra,
    #[cfg(target_os = "linux")]
    Forward,
    #[cfg(target_os = "linux")]
    Back,
    #[cfg(target_os = "linux")]
    Task,
}
