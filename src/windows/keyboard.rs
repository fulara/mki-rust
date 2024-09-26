use crate::Keyboard;
use std::convert::TryInto;
use std::mem::size_of;
use winapi::shared::minwindef::WORD;
use winapi::um::winuser::{
    INPUT_u, MapVirtualKeyW, SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP,
    KEYEVENTF_SCANCODE, LPINPUT, VK_ADD, VK_BACK, VK_CAPITAL, VK_DECIMAL, VK_DELETE, VK_DIVIDE,
    VK_DOWN, VK_ESCAPE, VK_F1, VK_F10, VK_F11, VK_F12, VK_F13, VK_F14, VK_F15, VK_F16, VK_F17,
    VK_F18, VK_F19, VK_F2, VK_F20, VK_F21, VK_F22, VK_F23, VK_F24, VK_F3, VK_F4, VK_F5, VK_F6,
    VK_F7, VK_F8, VK_F9, VK_HOME, VK_INSERT, VK_LCONTROL, VK_LEFT, VK_LMENU, VK_LSHIFT, VK_LWIN,
    VK_MULTIPLY, VK_NEXT, VK_NUMLOCK, VK_NUMPAD0, VK_NUMPAD1, VK_NUMPAD2, VK_NUMPAD3, VK_NUMPAD4,
    VK_NUMPAD5, VK_NUMPAD6, VK_NUMPAD7, VK_NUMPAD8, VK_NUMPAD9, VK_OEM_1, VK_OEM_2, VK_OEM_3,
    VK_OEM_4, VK_OEM_5, VK_OEM_6, VK_OEM_7, VK_OEM_COMMA, VK_OEM_PERIOD, VK_PRINT, VK_PRIOR,
    VK_RCONTROL, VK_RETURN, VK_RIGHT, VK_RMENU, VK_RSHIFT, VK_RWIN, VK_SCROLL, VK_SEPARATOR,
    VK_SNAPSHOT, VK_SPACE, VK_SUBTRACT, VK_TAB, VK_UP, VK_OEM_102
};

pub(crate) mod kimpl {
    use crate::windows::keyboard::{send_key_stroke, vk_code};
    use crate::Keyboard;
    use winapi::um::winuser::GetKeyState;

    pub(crate) fn press(key: Keyboard) {
        send_key_stroke(true, key)
    }

    pub(crate) fn release(key: Keyboard) {
        send_key_stroke(false, key)
    }

    pub(crate) fn click(key: Keyboard) {
        // Do we need sleep in between?
        press(key);
        release(key);
    }

    pub(crate) fn is_toggled(key: Keyboard) -> bool {
        // GetAsync is universal, but does not provide whether button is toggled.
        // as the GetKeyState seems to guarantee the correctness.
        let state = unsafe { GetKeyState(vk_code(key).into()) };
        i32::from(state) & 0x8001 != 0
    }
}

pub fn send_key_stroke(press: bool, key: Keyboard) {
    let action = if press {
        0 // 0 means to press.
    } else {
        KEYEVENTF_KEYUP
    };
    unsafe {
        let mut input_u: INPUT_u = std::mem::zeroed();
        *input_u.ki_mut() = KEYBDINPUT {
            wVk: 0,
            wScan: MapVirtualKeyW(vk_code(key).into(), 0)
                .try_into()
                .expect("Failed to map vk to scan code"), // This ignores the keyboard layout so better than vk?
            dwFlags: KEYEVENTF_SCANCODE | action,
            time: 0,
            dwExtraInfo: 0,
        };

        let mut x = INPUT {
            type_: INPUT_KEYBOARD,
            u: input_u,
        };

        SendInput(1, &mut x as LPINPUT, size_of::<INPUT>() as libc::c_int);
    }
}

// those dont have defines.
const VK_0: i32 = 0x30;
const VK_1: i32 = 0x31;
const VK_2: i32 = 0x32;
const VK_3: i32 = 0x33;
const VK_4: i32 = 0x34;
const VK_5: i32 = 0x35;
const VK_6: i32 = 0x36;
const VK_7: i32 = 0x37;
const VK_8: i32 = 0x38;
const VK_9: i32 = 0x39;
const VK_A: i32 = 0x41;
const VK_B: i32 = 0x42;
const VK_C: i32 = 0x43;
const VK_D: i32 = 0x44;
const VK_E: i32 = 0x45;
const VK_F: i32 = 0x46;
const VK_G: i32 = 0x47;
const VK_H: i32 = 0x48;
const VK_I: i32 = 0x49;
const VK_J: i32 = 0x4A;
const VK_K: i32 = 0x4B;
const VK_L: i32 = 0x4C;
const VK_M: i32 = 0x4D;
const VK_N: i32 = 0x4E;
const VK_O: i32 = 0x4F;
const VK_P: i32 = 0x50;
const VK_Q: i32 = 0x51;
const VK_R: i32 = 0x52;
const VK_S: i32 = 0x53;
const VK_T: i32 = 0x54;
const VK_U: i32 = 0x55;
const VK_V: i32 = 0x56;
const VK_W: i32 = 0x57;
const VK_X: i32 = 0x58;
const VK_Y: i32 = 0x59;
const VK_Z: i32 = 0x5A;

fn vk_code(key: Keyboard) -> WORD {
    i32::from(key)
        .try_into()
        .expect("vk does not fit into WORD")
}

impl From<Keyboard> for i32 {
    fn from(key: Keyboard) -> i32 {
        use Keyboard::*;
        match key {
            BackSpace => VK_BACK,
            Tab => VK_TAB,
            Enter => VK_RETURN,
            Escape => VK_ESCAPE,
            Space => VK_SPACE,
            PageUp => VK_PRIOR,
            PageDown => VK_NEXT,
            Home => VK_HOME,
            Left => VK_LEFT,
            Up => VK_UP,
            Right => VK_RIGHT,
            Down => VK_DOWN,
            Print => VK_PRINT,
            PrintScreen => VK_SNAPSHOT,
            Insert => VK_INSERT,
            Delete => VK_DELETE,
            Number0 => VK_0,
            Number1 => VK_1,
            Number2 => VK_2,
            Number3 => VK_3,
            Number4 => VK_4,
            Number5 => VK_5,
            Number6 => VK_6,
            Number7 => VK_7,
            Number8 => VK_8,
            Number9 => VK_9,
            A => VK_A,
            B => VK_B,
            C => VK_C,
            D => VK_D,
            E => VK_E,
            F => VK_F,
            G => VK_G,
            H => VK_H,
            I => VK_I,
            J => VK_J,
            K => VK_K,
            L => VK_L,
            M => VK_M,
            N => VK_N,
            O => VK_O,
            P => VK_P,
            Q => VK_Q,
            R => VK_R,
            S => VK_S,
            T => VK_T,
            U => VK_U,
            V => VK_V,
            W => VK_W,
            X => VK_X,
            Y => VK_Y,
            Z => VK_Z,
            LeftWindows => VK_LWIN,
            RightWindows => VK_RWIN,
            Numpad0 => VK_NUMPAD0,
            Numpad1 => VK_NUMPAD1,
            Numpad2 => VK_NUMPAD2,
            Numpad3 => VK_NUMPAD3,
            Numpad4 => VK_NUMPAD4,
            Numpad5 => VK_NUMPAD5,
            Numpad6 => VK_NUMPAD6,
            Numpad7 => VK_NUMPAD7,
            Numpad8 => VK_NUMPAD8,
            Numpad9 => VK_NUMPAD9,
            Multiply => VK_MULTIPLY,
            Add => VK_ADD,
            Separator => VK_SEPARATOR,
            Subtract => VK_SUBTRACT,
            Decimal => VK_DECIMAL,
            Divide => VK_DIVIDE,
            F1 => VK_F1,
            F2 => VK_F2,
            F3 => VK_F3,
            F4 => VK_F4,
            F5 => VK_F5,
            F6 => VK_F6,
            F7 => VK_F7,
            F8 => VK_F8,
            F9 => VK_F9,
            F10 => VK_F10,
            F11 => VK_F11,
            F12 => VK_F12,
            F13 => VK_F13,
            F14 => VK_F14,
            F15 => VK_F15,
            F16 => VK_F16,
            F17 => VK_F17,
            F18 => VK_F18,
            F19 => VK_F19,
            F20 => VK_F20,
            F21 => VK_F21,
            F22 => VK_F22,
            F23 => VK_F23,
            F24 => VK_F24,
            NumLock => VK_NUMLOCK,
            ScrollLock => VK_SCROLL,
            CapsLock => VK_CAPITAL,
            LeftShift => VK_LSHIFT,
            RightShift => VK_RSHIFT,
            LeftControl => VK_LCONTROL,
            RightControl => VK_RCONTROL,
            LeftAlt => VK_LMENU,
            RightAlt => VK_RMENU,
            Other(code) => code,
            Comma => VK_OEM_COMMA,
            Period => VK_OEM_PERIOD,
            Slash => VK_OEM_2,
            SemiColon => VK_OEM_1,
            Grave => VK_OEM_3,
            LeftBrace => VK_OEM_4,
            BackwardSlash => VK_OEM_5,
            RightBrace => VK_OEM_6,
            Apostrophe => VK_OEM_7,
            ThatThingy => VK_OEM_102
        }
    }
}

impl From<i32> for Keyboard {
    fn from(code: i32) -> Self {
        use Keyboard::*;
        match code {
            VK_BACK => BackSpace,
            VK_TAB => Tab,
            VK_RETURN => Enter,
            VK_ESCAPE => Escape,
            VK_SPACE => Space,
            VK_PRIOR => PageUp,
            VK_NEXT => PageDown,
            VK_HOME => Home,
            VK_LEFT => Left,
            VK_UP => Up,
            VK_RIGHT => Right,
            VK_DOWN => Down,
            VK_PRINT => Print,
            VK_SNAPSHOT => PrintScreen,
            VK_INSERT => Insert,
            VK_DELETE => Delete,
            VK_0 => Number0,
            VK_1 => Number1,
            VK_2 => Number2,
            VK_3 => Number3,
            VK_4 => Number4,
            VK_5 => Number5,
            VK_6 => Number6,
            VK_7 => Number7,
            VK_8 => Number8,
            VK_9 => Number9,
            VK_A => A,
            VK_B => B,
            VK_C => C,
            VK_D => D,
            VK_E => E,
            VK_F => F,
            VK_G => G,
            VK_H => H,
            VK_I => I,
            VK_J => J,
            VK_K => K,
            VK_L => L,
            VK_M => M,
            VK_N => N,
            VK_O => O,
            VK_P => P,
            VK_Q => Q,
            VK_R => R,
            VK_S => S,
            VK_T => T,
            VK_U => U,
            VK_V => V,
            VK_W => W,
            VK_X => X,
            VK_Y => Y,
            VK_Z => Z,
            VK_LWIN => LeftWindows,
            VK_RWIN => RightWindows,
            VK_NUMPAD0 => Numpad0,
            VK_NUMPAD1 => Numpad1,
            VK_NUMPAD2 => Numpad2,
            VK_NUMPAD3 => Numpad3,
            VK_NUMPAD4 => Numpad4,
            VK_NUMPAD5 => Numpad5,
            VK_NUMPAD6 => Numpad6,
            VK_NUMPAD7 => Numpad7,
            VK_NUMPAD8 => Numpad8,
            VK_NUMPAD9 => Numpad9,
            VK_MULTIPLY => Multiply,
            VK_ADD => Add,
            VK_SEPARATOR => Separator,
            VK_SUBTRACT => Subtract,
            VK_DECIMAL => Decimal,
            VK_DIVIDE => Divide,
            VK_F1 => F1,
            VK_F2 => F2,
            VK_F3 => F3,
            VK_F4 => F4,
            VK_F5 => F5,
            VK_F6 => F6,
            VK_F7 => F7,
            VK_F8 => F8,
            VK_F9 => F9,
            VK_F10 => F10,
            VK_F11 => F11,
            VK_F12 => F12,
            VK_F13 => F13,
            VK_F14 => F14,
            VK_F15 => F15,
            VK_F16 => F16,
            VK_F17 => F17,
            VK_F18 => F18,
            VK_F19 => F19,
            VK_F20 => F20,
            VK_F21 => F21,
            VK_F22 => F22,
            VK_F23 => F23,
            VK_F24 => F24,
            VK_NUMLOCK => NumLock,
            VK_SCROLL => ScrollLock,
            VK_CAPITAL => CapsLock,
            VK_LSHIFT => LeftShift,
            VK_RSHIFT => RightShift,
            VK_LCONTROL => LeftControl,
            VK_RCONTROL => RightControl,
            VK_LMENU => LeftAlt,
            VK_RMENU => RightAlt,
            VK_OEM_PERIOD => Period,
            VK_OEM_COMMA => Comma,
            VK_OEM_1 => SemiColon,
            VK_OEM_2 => Slash,
            VK_OEM_3 => Grave,
            VK_OEM_4 => LeftBrace,
            VK_OEM_5 => BackwardSlash,
            VK_OEM_6 => RightBrace,
            VK_OEM_7 => Apostrophe,
            _ => Other(code),
        }
    }
}
