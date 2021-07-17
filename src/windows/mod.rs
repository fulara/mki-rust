// pub mod hotkey;
pub mod keyboard;
pub mod mouse;

use crate::details::lock_registry;
use crate::{InhibitEvent, KeybdKey};
use std::convert::TryInto;
use std::mem::MaybeUninit;
use std::ptr::null_mut;
use winapi::shared::minwindef::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use winapi::shared::windef::HHOOK__;
use winapi::um::winuser::{
    CallNextHookEx, GetMessageW, SetWindowsHookExW, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL,
    WM_KEYDOWN,
};

pub(crate) fn install_hooks() {
    install_hook(WH_KEYBOARD_LL, keybd_hook);
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
        inhibit = lock_registry().key_down(key);
    }

    if inhibit == InhibitEvent::Yes {
        1
    } else {
        CallNextHookEx(null_mut(), code, w_param, l_param)
    }
}
