use crate::Mouse;
use std::mem::{size_of, transmute_copy};
use winapi::um::winuser::{
    SendInput, INPUT, INPUT_MOUSE, LPINPUT, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN,
    MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_RIGHTDOWN,
    MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_XDOWN, MOUSEEVENTF_XUP, MOUSEINPUT, XBUTTON1, XBUTTON2,
};

pub(crate) mod mimpl {
    use crate::windows::mouse::{mouse_click, mouse_press, mouse_release};
    use crate::Mouse;

    pub(crate) fn press(button: Mouse) {
        mouse_press(button)
    }

    pub(crate) fn click(button: Mouse) {
        mouse_click(button);
    }

    pub(crate) fn release(button: Mouse) {
        mouse_release(button);
    }
}

struct Pos {
    x: i32,
    y: i32,
}

fn mouse_interact_with(mut interaction: u32, mouse_data: u16, pos: Option<Pos>) {
    let mut x = 0;
    let mut y = 0;
    if let Some(pos) = pos {
        interaction |= MOUSEEVENTF_ABSOLUTE;
        x = pos.x;
        y = pos.y;
    }
    unsafe {
        let mut x = INPUT {
            type_: INPUT_MOUSE,
            u: transmute_copy(&MOUSEINPUT {
                dx: x,
                dy: y,
                mouseData: mouse_data.into(),
                time: 0,
                dwFlags: interaction,
                dwExtraInfo: 0,
            }),
        };

        SendInput(1, &mut x as LPINPUT, size_of::<INPUT>() as libc::c_int);
    }
}

pub fn mouse_press(button: Mouse) {
    let click = button_to_event_down(button) | button_to_event_up(button);
    mouse_interact_with(click, button_to_mouse_data(button), mouse_to_pos(button))
}

pub fn mouse_release(button: Mouse) {
    mouse_interact_with(
        button_to_event_up(button),
        button_to_mouse_data(button),
        mouse_to_pos(button),
    )
}

pub fn mouse_click(button: Mouse) {
    mouse_interact_with(
        button_to_event_down(button),
        button_to_mouse_data(button),
        mouse_to_pos(button),
    )
}

fn button_to_mouse_data(button: Mouse) -> u16 {
    match button {
        Mouse::Side | Mouse::DoubleSide => XBUTTON1,
        Mouse::Extra | Mouse::DoubleExtra => XBUTTON2,
        _ => 0,
    }
}

fn button_to_event_up(button: Mouse) -> u32 {
    use Mouse::*;
    match button {
        Left | DoubleLeft => MOUSEEVENTF_LEFTDOWN,
        Right | DoubleRight => MOUSEEVENTF_RIGHTDOWN,
        Middle | DoubleMiddle => MOUSEEVENTF_MIDDLEDOWN,
        Side | DoubleSide | Extra | DoubleExtra => MOUSEEVENTF_XDOWN,
    }
}

fn button_to_event_down(button: Mouse) -> u32 {
    use Mouse::*;
    match button {
        Left | DoubleLeft => MOUSEEVENTF_LEFTUP,
        Right | DoubleRight => MOUSEEVENTF_RIGHTUP,
        Middle | DoubleMiddle => MOUSEEVENTF_MIDDLEUP,
        Side | DoubleSide | Extra | DoubleExtra => MOUSEEVENTF_XUP,
    }
}

fn mouse_to_pos(button: Mouse) -> Option<Pos> {
    use Mouse::*;
    match button {
        Left | DoubleLeft => None,
        Right | DoubleRight => None,
        Middle | DoubleMiddle => None,
        Side | DoubleSide | Extra | DoubleExtra => None,
    }
}

pub fn move_mouse_impl(x: i32, y: i32) {
    mouse_interact_with(0, 0, Some(Pos { x, y }));
}
