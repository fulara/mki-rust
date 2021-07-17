#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug)]
pub enum MouseButton {
    // TODO: double clicks.
    Left,
    #[cfg(target_os = "windows")] // Not sure how to detect double on linux
    DoubleLeft,
    Right,
    #[cfg(target_os = "windows")] // Not sure how to detect double on linux
    DoubleRight,
    Middle,
    #[cfg(target_os = "windows")] // Not sure how to detect double on linux
    DoubleMiddle,
    Side, // XBUTTON1 on win
    #[cfg(target_os = "windows")] // Not sure how to detect double on linux
    DoubleSide,
    Extra, // XBUTTON2 on win
    #[cfg(target_os = "windows")] // Not sure how to detect double on linux
    DoubleExtra,
    #[cfg(target_os = "linux")]
    Forward,
    #[cfg(target_os = "linux")]
    Back,
    #[cfg(target_os = "linux")]
    Task,
}
