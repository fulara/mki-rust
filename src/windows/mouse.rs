use std::mem::{size_of, transmute_copy};
use user32::SendInput;

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

pub fn mouse_click_left() {
    mouse_interact_with(MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_LEFTUP);
}

pub fn mouse_down_left() {
    mouse_interact_with(MOUSEEVENTF_LEFTDOWN);
}

pub fn mouse_up_left() {
    mouse_interact_with(MOUSEEVENTF_LEFTUP);
}

pub fn mouse_click_right() {
    mouse_interact_with(MOUSEEVENTF_RIGHTDOWN | MOUSEEVENTF_RIGHTUP);
}

pub fn mouse_down_right() {
    mouse_interact_with(MOUSEEVENTF_RIGHTDOWN);
}

pub fn mouse_up_right() {
    mouse_interact_with(MOUSEEVENTF_RIGHTUP);
}
