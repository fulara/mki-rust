use crate::{Button, MouseButton};
use std::mem::{size_of, transmute_copy};
use winapi::um::winuser::{
    GetAsyncKeyState, SendInput, INPUT, INPUT_MOUSE, LPINPUT, MOUSEEVENTF_LEFTDOWN,
    MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_RIGHTDOWN,
    MOUSEEVENTF_RIGHTUP, MOUSEINPUT, VK_LBUTTON, VK_MBUTTON, VK_RBUTTON, VK_XBUTTON1, VK_XBUTTON2,
};

impl Button for MouseButton {
    fn press(&self) {
        mouse_press(*self)
    }

    fn click(&self) {
        mouse_click(*self);
    }

    fn release(&self) {
        mouse_release(*self);
    }

    fn is_pressed(&self) -> bool {
        mouse_is_down(*self)
    }
}

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

        SendInput(1, &mut x as LPINPUT, size_of::<INPUT>() as libc::c_int);
    }
}

pub fn mouse_press(button: MouseButton) {
    let interaction = match button {
        MouseButton::Left => MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_LEFTUP,
        MouseButton::Right => MOUSEEVENTF_RIGHTDOWN | MOUSEEVENTF_RIGHTUP,
        MouseButton::Middle => MOUSEEVENTF_MIDDLEDOWN | MOUSEEVENTF_MIDDLEUP,
        MouseButton::Side => todo!("requires filling of dwflags"),
        MouseButton::Extra => todo!("requires filling of dwflags"),
    };
    mouse_interact_with(interaction)
}

pub fn mouse_release(button: MouseButton) {
    let interaction = match button {
        MouseButton::Left => MOUSEEVENTF_LEFTUP,
        MouseButton::Right => MOUSEEVENTF_RIGHTUP,
        MouseButton::Middle => MOUSEEVENTF_MIDDLEUP,
        MouseButton::Side => todo!("requires filling of dwflags"),
        MouseButton::Extra => todo!("requires filling of dwflags"),
    };
    mouse_interact_with(interaction)
}

pub fn mouse_click(button: MouseButton) {
    let interaction = match button {
        MouseButton::Left => MOUSEEVENTF_LEFTUP,
        MouseButton::Right => MOUSEEVENTF_RIGHTUP,
        MouseButton::Middle => MOUSEEVENTF_MIDDLEUP,
        MouseButton::Side => todo!("requires filling of dwflags"),
        MouseButton::Extra => todo!("requires filling of dwflags"),
    };
    mouse_interact_with(interaction)
}

pub fn mouse_is_down(button: MouseButton) -> bool {
    let vk = match button {
        MouseButton::Left => VK_LBUTTON,
        MouseButton::Right => VK_RBUTTON,
        MouseButton::Middle => VK_MBUTTON,
        MouseButton::Side => VK_XBUTTON1,
        MouseButton::Extra => VK_XBUTTON2,
    };
    let state = unsafe { GetAsyncKeyState(vk) };
    i32::from(state) & 0x8000 != 0
}
