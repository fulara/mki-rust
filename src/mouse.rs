use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")] // Not sure how to detect double on linux
#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Mouse {
    Left,
    DoubleLeft,
    Right,
    DoubleRight,
    Middle,
    DoubleMiddle,
    Side, // XBUTTON1
    DoubleSide,
    Extra, // XBUTTON2
    DoubleExtra,
}

#[cfg(target_os = "linux")]
#[derive(Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Mouse {
    Left,
    Right,
    Middle,
    Side,
    Extra,
    Forward,
    Back,
    Task,
}
