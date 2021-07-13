use winapi::*;

use std::mem::{size_of, transmute_copy};
use user32::SendInput;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Keys {
    Return,
    Control,
    Alt,
    Shift,
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
}

pub fn send_key_stroke(p: Keys) {
    unsafe {
        let mut x = INPUT {
            type_: INPUT_KEYBOARD,
            u: transmute_copy(&KEYBDINPUT {
                wVk: get_vk_code(p) as WORD, // 'a' key
                wScan: 0,                    // 0 := hardware scan code for a key
                dwFlags: 0,                  // 0 := a key press
                time: 0,
                dwExtraInfo: 0,
            }),
        };

        SendInput(1, &mut x as LPINPUT, size_of::<INPUT>() as c_int);
    }
}

fn get_vk_code(p: Keys) -> WORD {
    use self::Keys::*;
    match p {
        A => 'A' as WORD,
        B => 'B' as WORD,
        C => 'C' as WORD,
        D => 'D' as WORD,
        E => 'E' as WORD,
        F => 'F' as WORD,
        G => 'G' as WORD,
        H => 'H' as WORD,
        I => 'I' as WORD,
        J => 'J' as WORD,
        K => 'K' as WORD,
        L => 'L' as WORD,
        M => 'M' as WORD,
        N => 'N' as WORD,
        O => 'O' as WORD,
        P => 'P' as WORD,
        Q => 'Q' as WORD,
        R => 'R' as WORD,
        S => 'S' as WORD,
        T => 'T' as WORD,
        U => 'U' as WORD,
        V => 'V' as WORD,
        W => 'W' as WORD,
        X => 'X' as WORD,
        Y => 'Y' as WORD,
        Z => 'Z' as WORD,
        Return => VK_RETURN as WORD,
        Shift => VK_SHIFT as WORD,
        Control => VK_CONTROL as WORD,
        Alt => VK_MENU as WORD,
    }
}
