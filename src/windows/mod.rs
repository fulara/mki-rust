// pub mod hotkey;
pub mod keyboard;
pub mod mouse;

use crate::details::lock_registry;
use crate::{Event, InhibitEvent, KeybdKey, MouseButton};
use std::convert::TryInto;
use std::mem::MaybeUninit;
use std::ptr::null_mut;
use winapi::shared::minwindef::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use winapi::shared::windef::HHOOK__;
use winapi::um::winuser::{
    CallNextHookEx, GetMessageW, SetWindowsHookExW, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL,
    WH_MOUSE_LL, WM_KEYDOWN, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP,
    WM_RBUTTONDOWN, WM_RBUTTONUP, WM_XBUTTONDOWN, WM_XBUTTONUP, XBUTTON1, XBUTTON2,
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
    let mut inhibit = InhibitEvent::No;
    // Note this seemingly is only activated when ALT is not pressed, need to handle WM_SYSKEYDOWN then
    // Test that case.
    if w_param as u32 == WM_KEYDOWN {
        let vk: i32 = (*(l_param as *const KBDLLHOOKSTRUCT))
            .vkCode
            .try_into()
            .expect("vkCode does not fit in i32");
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
        let key: KeybdKey = vk.into();
        inhibit = lock_registry().event_down(Event::Keyboard(key));
    }

    if inhibit == InhibitEvent::Yes {
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
    let maybe_x_button = if data.mouseData == XBUTTON1.into() {
        Some(MouseButton::Side)
    } else if data.mouseData == XBUTTON2.into() {
        Some(MouseButton::Extra)
    } else {
        None
    };
    let w_param_u32: u32 = w_param.try_into().expect("w_param > u32");
    let inhibit = match w_param_u32 {
        code if code == WM_LBUTTONDOWN => {
            lock_registry().event_down(Event::Mouse(MouseButton::Left))
        }
        code if code == WM_LBUTTONDBLCLK => {
            lock_registry().event_click(Event::Mouse(MouseButton::DoubleLeft))
        }
        code if code == WM_RBUTTONDOWN => {
            lock_registry().event_down(Event::Mouse(MouseButton::Right))
        }
        code if code == WM_RBUTTONDBLCLK => {
            lock_registry().event_click(Event::Mouse(MouseButton::DoubleRight))
        }
        code if code == WM_MBUTTONDOWN => {
            lock_registry().event_down(Event::Mouse(MouseButton::Middle))
        }
        code if code == WM_MBUTTONDBLCLK => {
            lock_registry().event_down(Event::Mouse(MouseButton::DoubleMiddle))
        }
        code if code == WM_XBUTTONDOWN => {
            lock_registry().event_down(Event::Mouse(maybe_x_button.unwrap()))
        }
        code if code == WM_XBUTTONDBLCLK => {
            if MouseButton::Side == maybe_x_button.unwrap() {
                lock_registry().event_click(Event::Mouse(MouseButton::DoubleSide))
            } else {
                lock_registry().event_click(Event::Mouse(MouseButton::DoubleExtra))
            }
        }
        code if code == WM_LBUTTONUP => lock_registry().event_up(Event::Mouse(MouseButton::Left)),
        code if code == WM_LBUTTONUP => lock_registry().event_up(Event::Mouse(MouseButton::Left)),
        code if code == WM_RBUTTONUP => lock_registry().event_up(Event::Mouse(MouseButton::Right)),
        code if code == WM_RBUTTONUP => lock_registry().event_up(Event::Mouse(MouseButton::Right)),
        code if code == WM_MBUTTONUP => lock_registry().event_up(Event::Mouse(MouseButton::Middle)),
        code if code == WM_MBUTTONUP => lock_registry().event_up(Event::Mouse(MouseButton::Middle)),
        code if code == WM_XBUTTONUP => {
            lock_registry().event_up(Event::Mouse(maybe_x_button.unwrap()))
        }
        code if code == WM_XBUTTONUP => {
            lock_registry().event_up(Event::Mouse(maybe_x_button.unwrap()))
        }
        _ => InhibitEvent::No,
    };
    if inhibit == InhibitEvent::Yes {
        1
    } else {
        CallNextHookEx(null_mut(), code, w_param, l_param)
    }
}
