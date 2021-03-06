use crate::{Key, KeybdKey};
use std::convert::TryInto;
use std::mem::{size_of, transmute_copy};
use winapi::shared::minwindef::WORD;
use winapi::um::winuser::{
    GetAsyncKeyState, GetKeyState, MapVirtualKeyW, SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT,
    KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, LPINPUT,
};

impl Key for KeybdKey {
    fn press(&self) {
        send_key_stroke(true, *self)
    }

    fn release(&self) {
        send_key_stroke(false, *self)
    }

    fn is_pressed(&self) -> bool {
        let state = unsafe { GetAsyncKeyState(vk_code(*self).into()) };
        i32::from(state) & 0x8000 != 0
    }

    fn is_toggled(&self) -> bool {
        // GetAsync is universal, but does not provide whether button is toggled.
        // as the GetKeyState seems to guarantee the correctness.
        let state = unsafe { GetKeyState(vk_code(*self).into()) };
        i32::from(state) & 0x8001 != 0
    }
}

pub fn send_key_stroke(press: bool, key: KeybdKey) {
    let action = if press {
        0 // 0 means to press.
    } else {
        KEYEVENTF_KEYUP
    };
    unsafe {
        let mut x = INPUT {
            type_: INPUT_KEYBOARD,
            u: transmute_copy(&KEYBDINPUT {
                wVk: 0,
                wScan: MapVirtualKeyW(vk_code(key).into(), 0)
                    .try_into()
                    .expect("Failed to map vk to scan code"), // This ignores the keyboard layout so better than vk?
                dwFlags: KEYEVENTF_SCANCODE | action,
                time: 0,
                dwExtraInfo: 0,
            }),
        };

        SendInput(1, &mut x as LPINPUT, size_of::<INPUT>() as libc::c_int);
    }
}

fn vk_code(key: KeybdKey) -> WORD {
    use KeybdKey::*;

    let vk: u16 = match key {
        Backspace => 0x08,
        Tab => 0x09,
        Enter => 0x0D,
        Escape => 0x1B,
        Space => 0x20,
        Home => 0x24,
        Left => 0x25,
        Up => 0x26,
        Right => 0x27,
        Down => 0x28,
        Insert => 0x2D,
        Delete => 0x2E,
        Numrow0 => 0x30,
        Numrow1 => 0x31,
        Numrow2 => 0x32,
        Numrow3 => 0x33,
        Numrow4 => 0x34,
        Numrow5 => 0x35,
        Numrow6 => 0x36,
        Numrow7 => 0x37,
        Numrow8 => 0x38,
        Numrow9 => 0x39,
        A => 0x41,
        B => 0x42,
        C => 0x43,
        D => 0x44,
        E => 0x45,
        F => 0x46,
        G => 0x47,
        H => 0x48,
        I => 0x49,
        J => 0x4A,
        K => 0x4B,
        L => 0x4C,
        M => 0x4D,
        N => 0x4E,
        O => 0x4F,
        P => 0x50,
        Q => 0x51,
        R => 0x52,
        S => 0x53,
        T => 0x54,
        U => 0x55,
        V => 0x56,
        W => 0x57,
        X => 0x58,
        Y => 0x59,
        Z => 0x5A,
        Numpad0 => 0x60,
        Numpad1 => 0x61,
        Numpad2 => 0x62,
        Numpad3 => 0x63,
        Numpad4 => 0x64,
        Numpad5 => 0x65,
        Numpad6 => 0x66,
        Numpad7 => 0x67,
        Numpad8 => 0x68,
        Numpad9 => 0x69,
        F1 => 0x70,
        F2 => 0x71,
        F3 => 0x72,
        F4 => 0x73,
        F5 => 0x74,
        F6 => 0x75,
        F7 => 0x76,
        F8 => 0x77,
        F9 => 0x78,
        F10 => 0x79,
        F11 => 0x7A,
        F12 => 0x7B,
        F13 => 0x7C,
        F14 => 0x7D,
        F15 => 0x7E,
        F16 => 0x7F,
        F17 => 0x80,
        F18 => 0x81,
        F19 => 0x82,
        F20 => 0x83,
        F21 => 0x84,
        F22 => 0x85,
        F23 => 0x86,
        F24 => 0x87,
        NumLock => 0x90,
        ScrollLock => 0x91,
        CapsLock => 0x14,
        LShift => 0xA0,
        RShift => 0xA1,
        LControl => 0xA2,
        RControl => 0xA3,
        Other(code) => code,
    };
    vk.into()
}

impl From<u16> for KeybdKey {
    fn from(code: u16) -> Self {
        use KeybdKey::*;
        match code {
            0x08 => Backspace,
            0x09 => Tab,
            0x0D => Enter,
            0x1B => Escape,
            0x20 => Space,
            0x24 => Home,
            0x25 => Left,
            0x26 => Up,
            0x27 => Right,
            0x28 => Down,
            0x2D => Insert,
            0x2E => Delete,
            0x30 => Numrow0,
            0x31 => Numrow1,
            0x32 => Numrow2,
            0x33 => Numrow3,
            0x34 => Numrow4,
            0x35 => Numrow5,
            0x36 => Numrow6,
            0x37 => Numrow7,
            0x38 => Numrow8,
            0x39 => Numrow9,
            0x41 => A,
            0x42 => B,
            0x43 => C,
            0x44 => D,
            0x45 => E,
            0x46 => F,
            0x47 => G,
            0x48 => H,
            0x49 => I,
            0x4A => J,
            0x4B => K,
            0x4C => L,
            0x4D => M,
            0x4E => N,
            0x4F => O,
            0x50 => P,
            0x51 => Q,
            0x52 => R,
            0x53 => S,
            0x54 => T,
            0x55 => U,
            0x56 => V,
            0x57 => W,
            0x58 => X,
            0x59 => Y,
            0x5A => Z,
            0x60 => Numpad0,
            0x61 => Numpad1,
            0x62 => Numpad2,
            0x63 => Numpad3,
            0x64 => Numpad4,
            0x65 => Numpad5,
            0x66 => Numpad6,
            0x67 => Numpad7,
            0x68 => Numpad8,
            0x69 => Numpad9,
            0x70 => F1,
            0x71 => F2,
            0x72 => F3,
            0x73 => F4,
            0x74 => F5,
            0x75 => F6,
            0x76 => F7,
            0x77 => F8,
            0x78 => F9,
            0x79 => F10,
            0x7A => F11,
            0x7B => F12,
            0x7C => F13,
            0x7D => F14,
            0x7E => F15,
            0x7F => F16,
            0x80 => F17,
            0x81 => F18,
            0x82 => F19,
            0x83 => F20,
            0x84 => F21,
            0x85 => F22,
            0x86 => F23,
            0x87 => F24,
            0x90 => NumLock,
            0x91 => ScrollLock,
            0x14 => CapsLock,
            0xA0 => LShift,
            0xA1 => RShift,
            0xA2 => LControl,
            0xA3 => RControl,
            _ => Other(code),
        }
    }
}
