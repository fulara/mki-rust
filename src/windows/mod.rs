pub mod keyboard;
pub mod mouse;

use crate::details::registry;
use crate::{Event, InhibitEvent, Keyboard, Mouse};
use std::convert::TryInto;
use std::mem::MaybeUninit;
use std::ptr::null_mut;
use winapi::shared::minwindef::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use winapi::shared::windef::HHOOK__;
use winapi::um::winuser::{
    CallNextHookEx, GetMessageW, SetWindowsHookExW, GET_XBUTTON_WPARAM, KBDLLHOOKSTRUCT, MSG,
    WH_KEYBOARD_LL, WH_MOUSE_LL, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP,
    WM_MBUTTONDOWN, WM_MBUTTONUP, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
    WM_XBUTTONDOWN, WM_XBUTTONUP, XBUTTON1, XBUTTON2,
};
use winapi::um::winuser::{
    MSLLHOOKSTRUCT, WM_LBUTTONDBLCLK, WM_MBUTTONDBLCLK, WM_RBUTTONDBLCLK, WM_XBUTTONDBLCLK,
};

pub(crate) fn install_hooks() {
    install_hook(WH_KEYBOARD_LL, keybd_hook);
    install_hook(WH_MOUSE_LL, mouse_hook);
}

pub(crate) fn process_message() {
    let mut msg: MSG = unsafe { MaybeUninit::zeroed().assume_init() };
    unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
}

fn install_hook(
    hook_id: libc::c_int,
    hook_proc: unsafe extern "system" fn(libc::c_int, WPARAM, LPARAM) -> LRESULT,
) -> *mut HHOOK__ {
    unsafe { SetWindowsHookExW(hook_id, Some(hook_proc), 0 as HINSTANCE, 0) }
}

unsafe extern "system" fn keybd_hook(
    code: libc::c_int,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    let hook_struct = *(l_param as *const KBDLLHOOKSTRUCT);
    let vk: i32 = hook_struct.vkCode.try_into().expect("vkCode does not fit in i32");
    if hook_struct.time == 1 {
        return CallNextHookEx(null_mut(), code, w_param, l_param);
    }
    // https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-keydown
    // Says that we can find the repeat bit here, however that does not apply to lowlvlkb hook which this is.
    // Because IDE is not capable of following to the definition here it is:
    // STRUCT!{struct KBDLLHOOKSTRUCT {
    //     vkCode: DWORD,
    //     scanCode: DWORD,
    //     flags: DWORD,
    //     time: DWORD,
    //     dwExtraInfo: ULONG_PTR,
    // }}

    let mut inhibit = InhibitEvent::No;
    // Note this seemingly is only activated when ALT is not pressed, need to handle WM_SYSKEYDOWN then
    // Test that case.
    let key: Keyboard = vk.into();
    match w_param as u32 {
        code if code == WM_KEYDOWN || code == WM_SYSKEYDOWN => {
            inhibit = registry().event_down(Event::Keyboard(key));
        }
        code if code == WM_KEYUP || code == WM_SYSKEYUP => {
            inhibit = registry().event_up(Event::Keyboard(key));
        }
        _ => {}
    }

    if inhibit.should_inhibit() {
        1
    } else {
        CallNextHookEx(null_mut(), code, w_param, l_param)
    }
}

unsafe extern "system" fn mouse_hook(
    code: libc::c_int,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    // because macros > idea
    // typedef struct tagMSLLHOOKSTRUCT {
    //   POINT     pt;
    //   DWORD     mouseData;
    //   DWORD     flags;
    //   DWORD     time;
    //   ULONG_PTR dwExtraInfo;
    // } MSLLHOOKSTRUCT, *LPMSLLHOOKSTRUCT, *PMSLLHOOKSTRUCT;

    let data = &*(l_param as *const MSLLHOOKSTRUCT);
    let x_button_param: u16 =
        GET_XBUTTON_WPARAM(data.mouseData.try_into().expect("u32 fits usize"));
    let maybe_x_button = if x_button_param == XBUTTON1 {
        Some(Mouse::Side)
    } else if x_button_param == XBUTTON2 {
        Some(Mouse::Extra)
    } else {
        None
    };
    let w_param_u32: u32 = w_param.try_into().expect("w_param > u32");
    registry().update_mouse_position(data.pt.x, data.pt.y);
    let inhibit = match w_param_u32 {
        code if code == WM_LBUTTONDOWN => registry().event_down(Event::Mouse(Mouse::Left)),
        code if code == WM_LBUTTONDBLCLK => registry().event_click(Event::Mouse(Mouse::DoubleLeft)),
        code if code == WM_RBUTTONDOWN => registry().event_down(Event::Mouse(Mouse::Right)),
        code if code == WM_RBUTTONDBLCLK => {
            registry().event_click(Event::Mouse(Mouse::DoubleRight))
        }
        code if code == WM_MBUTTONDOWN => registry().event_down(Event::Mouse(Mouse::Middle)),
        code if code == WM_MBUTTONDBLCLK => {
            registry().event_down(Event::Mouse(Mouse::DoubleMiddle))
        }
        code if code == WM_XBUTTONDOWN => {
            if let Some(x_button) = maybe_x_button {
                registry().event_down(Event::Mouse(x_button))
            } else {
                InhibitEvent::No
            }
        }
        code if code == WM_XBUTTONDBLCLK => {
            if let Some(x_button) = maybe_x_button {
                // TODO: figure out the other XButtons.
                if Mouse::Side == x_button {
                    registry().event_click(Event::Mouse(Mouse::DoubleSide))
                } else {
                    registry().event_click(Event::Mouse(Mouse::DoubleExtra))
                }
            } else {
                InhibitEvent::No
            }
        }
        code if code == WM_LBUTTONUP => registry().event_up(Event::Mouse(Mouse::Left)),
        code if code == WM_LBUTTONUP => registry().event_up(Event::Mouse(Mouse::Left)),
        code if code == WM_RBUTTONUP => registry().event_up(Event::Mouse(Mouse::Right)),
        code if code == WM_RBUTTONUP => registry().event_up(Event::Mouse(Mouse::Right)),
        code if code == WM_MBUTTONUP => registry().event_up(Event::Mouse(Mouse::Middle)),
        code if code == WM_MBUTTONUP => registry().event_up(Event::Mouse(Mouse::Middle)),
        code if code == WM_XBUTTONUP => {
            if let Some(x_button) = maybe_x_button {
                registry().event_up(Event::Mouse(x_button))
            } else {
                InhibitEvent::No
            }
        }
        code if code == WM_XBUTTONUP => {
            if let Some(x_button) = maybe_x_button {
                registry().event_up(Event::Mouse(x_button))
            } else {
                InhibitEvent::No
            }
        }
        _ => InhibitEvent::No,
    };
    if inhibit.should_inhibit() {
        1
    } else {
        CallNextHookEx(null_mut(), code, w_param, l_param)
    }
}
