use crate::KeybdKey;
use std::sync::{Arc, Mutex, MutexGuard};
use uinput::event::keyboard::Key;
use uinput::event::relative::Position;
use uinput::event::Code;

fn device() -> MutexGuard<'static, uinput::Device> {
    lazy_static::lazy_static! {
        static ref DEVICE: Arc<Mutex<uinput::Device>> = Arc::new(Mutex::new(
            uinput::default()
            .unwrap()
            .name("mki")
            .unwrap()
            .event(uinput::event::Keyboard::All)
            .unwrap()
            .event(Position::X)
            .unwrap()
            .event(Position::Y)
            .unwrap()
            .create()
            .unwrap(),
        ));
    }
    DEVICE.lock().unwrap()
}
pub fn send_key_stroke(press: bool, key: KeybdKey) {
    // let action = if press {
    //     0 // 0 means to press.
    // } else {
    //     KEYEVENTF_KEYUP
    // };
    // unsafe {
    //     let mut x = INPUT {
    //         type_: INPUT_KEYBOARD,
    //         u: transmute_copy(&KEYBDINPUT {
    //             wVk: 0,
    //             wScan: MapVirtualKeyW(vk_code(key).into(), 0)
    //                 .try_into()
    //                 .expect("Failed to map vk to scan code"), // This ignores the keyboard layout so better than vk?
    //             dwFlags: KEYEVENTF_SCANCODE | action,
    //             time: 0,
    //             dwExtraInfo: 0,
    //         }),
    //     };
    //
    //     SendInput(1, &mut x as LPINPUT, size_of::<INPUT>() as libc::c_int);
    // }
}

impl From<KeybdKey> for u16 {
    fn from(key: KeybdKey) -> u16 {
        // Key::BackSpace.code()

        use KeybdKey::*;
        match key {
            Backspace => 0xFF08,
            Tab => 0xFF09,
            Enter => 0xFF8D,
            Escape => 0xFF1B,
            Space => 0x020,
            Home => 0xFF50,
            Left => 0xFF51,
            Up => 0xFF52,
            Right => 0xFF53,
            Down => 0xFF54,
            Insert => 0xFF63,
            Delete => 0xFF9F,
            Numrow0 => 0x030,
            Numrow1 => 0x031,
            Numrow2 => 0x032,
            Numrow3 => 0x033,
            Numrow4 => 0x034,
            Numrow5 => 0x035,
            Numrow6 => 0x036,
            Numrow7 => 0x037,
            Numrow8 => 0x038,
            Numrow9 => 0x039,
            A => 0x041,
            B => 0x042,
            C => 0x043,
            D => 0x044,
            E => 0x045,
            F => 0x046,
            G => 0x047,
            H => 0x048,
            I => 0x049,
            J => 0x04A,
            K => 0x04B,
            L => 0x04C,
            M => 0x04D,
            N => 0x04E,
            O => 0x04F,
            P => 0x050,
            Q => 0x051,
            R => 0x052,
            S => 0x053,
            T => 0x054,
            U => 0x055,
            V => 0x056,
            W => 0x057,
            X => 0x058,
            Y => 0x059,
            Z => 0x05A,
            Numpad0 => 0xFFB0,
            Numpad1 => 0xFFB1,
            Numpad2 => 0xFFB2,
            Numpad3 => 0xFFB3,
            Numpad4 => 0xFFB4,
            Numpad5 => 0xFFB5,
            Numpad6 => 0xFFB6,
            Numpad7 => 0xFFB7,
            Numpad8 => 0xFFB8,
            Numpad9 => 0xFFB9,
            F1 => 0xFFBE,
            F2 => 0xFFBF,
            F3 => 0xFFC0,
            F4 => 0xFFC1,
            F5 => 0xFFC2,
            F6 => 0xFFC3,
            F7 => 0xFFC4,
            F8 => 0xFFC5,
            F9 => 0xFFC6,
            F10 => 0xFFC7,
            F11 => 0xFFC8,
            F12 => 0xFFC9,
            F13 => 0xFFCA,
            F14 => 0xFFCB,
            F15 => 0xFFCC,
            F16 => 0xFFCD,
            F17 => 0xFFCE,
            F18 => 0xFFCF,
            F19 => 0xFFD0,
            F20 => 0xFFD1,
            F21 => 0xFFD2,
            F22 => 0xFFD3,
            F23 => 0xFFD4,
            F24 => 0xFFD5,
            NumLock => 0xFF7F,
            ScrollLock => 0xFF14,
            CapsLock => 0xFFE5,
            LShift => 0xFFE1,
            RShift => 0xFFE2,
            LControl => 0xFFE3,
            RControl => 0xFFE4,
            Other(code) => code,
        }
    }
}

// https://www.win.tue.nl/~aeb/linux/kbd/scancodes-1.html
pub fn key_to_scan_code(key: KeybdKey) -> i32 {
    use KeybdKey::*;
    match key {
        Backspace => 0x0e,
        Tab => 0x0f,
        Enter => 0x1c,
        Escape => 0x01,
        Space => 0x39,
        Home => 0x47,
        Left => 0x4b,
        Up => 0x48,
        Right => 0x4d,
        Down => 0x50,
        Insert => 0x52,
        Delete => 0x53,
        Numrow0 => 0x0b,
        Numrow1 => 0x02,
        Numrow2 => 0x03,
        Numrow3 => 0x04,
        Numrow4 => 0x05,
        Numrow5 => 0x06,
        Numrow6 => 0x07,
        Numrow7 => 0x08,
        Numrow8 => 0x09,
        Numrow9 => 0x0a,
        A => 0x1e,
        B => 0x30,
        C => 0x2e,
        D => 0x20,
        E => 0x12,
        F => 0x21,
        G => 0x22,
        H => 0x23,
        I => 0x17,
        J => 0x24,
        K => 0x25,
        L => 0x26,
        M => 0x32,
        N => 0x31,
        O => 0x18,
        P => 0x19,
        Q => 0x10,
        R => 0x13,
        S => 0x1f,
        T => 0x14,
        U => 0x16,
        V => 0x2f,
        W => 0x11,
        X => 0x2d,
        Y => 0x15,
        Z => 0x2c,
        Numpad0 => 0x52,
        Numpad1 => 0x4f,
        Numpad2 => 0x50,
        Numpad3 => 0x51,
        Numpad4 => 0x4b,
        Numpad5 => 0x4c,
        Numpad6 => 0x4d,
        Numpad7 => 0x47,
        Numpad8 => 0x48,
        Numpad9 => 0x49,
        F1 => 0x3b,
        F2 => 0x3c,
        F3 => 0x3d,
        F4 => 0x3e,
        F5 => 0x3f,
        F6 => 0x40,
        F7 => 0x41,
        F8 => 0x42,
        F9 => 0x43,
        F10 => 0x44,
        NumLock => 0x45,
        ScrollLock => 0x46,
        CapsLock => 0x3a,
        LShift => 0x2a,
        RShift => 0x36,
        LControl => 0x1d,
        Other(code) => code as i32,
        _ => 0x0,
    }
}
