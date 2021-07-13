use std::mem::{size_of, transmute_copy};
use user32::{GetKeyState, SendInput};

use crate::MouseButton;
use winapi::*;

fn mouse_interact_with(interaction: u32) {
    unsafe {
        let mut x = INPUT {
            type_: INPUT_MOUSE,
            u: transmute_copy(&MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouseData: 0,
                time: 0,
                dwFlags: interaction,
                dwExtraInfo: 0,
            }),
        };

        SendInput(1, &mut x as LPINPUT, size_of::<INPUT>() as c_int);
    }
}

pub fn mouse_down(button: MouseButton) {
    let interaction = match button {
        MouseButton::Left => MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_LEFTUP,
        MouseButton::Right => MOUSEEVENTF_RIGHTDOWN | MOUSEEVENTF_RIGHTUP,
        MouseButton::Middle => MOUSEEVENTF_MIDDLEDOWN | MOUSEEVENTF_MIDDLEUP,
    };
    mouse_interact_with(interaction)
}

pub fn mouse_up(button: MouseButton) {
    let interaction = match button {
        MouseButton::Left => MOUSEEVENTF_LEFTUP,
        MouseButton::Right => MOUSEEVENTF_RIGHTUP,
        MouseButton::Middle => MOUSEEVENTF_MIDDLEUP,
    };
    mouse_interact_with(interaction)
}

pub fn mouse_click(button: MouseButton) {
    let interaction = match button {
        MouseButton::Left => MOUSEEVENTF_LEFTUP,
        MouseButton::Right => MOUSEEVENTF_RIGHTUP,
        MouseButton::Middle => MOUSEEVENTF_MIDDLEUP,
    };
    mouse_interact_with(interaction)
}

pub fn mouse_is_down(button: MouseButton) -> bool {
    let vk = match button {
        MouseButton::Left => VK_LBUTTON,
        MouseButton::Right => VK_RBUTTON,
        MouseButton::Middle => VK_MBUTTON,
    };
    unsafe { i32::from(GetKeyState(vk)) & 0x8000 != 0 }
}
