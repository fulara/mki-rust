use std::ptr::null_mut;
use std::sync::atomic::AtomicPtr;
use std::thread;
use std::thread::JoinHandle;
use user32::{CallNextHookEx, SetWindowsHookExW};
use winapi::HINSTANCE;
use winapi::WH_KEYBOARD_LL;
use winapi::{HHOOK, LPARAM, LRESULT, WPARAM};

pub mod hotkey;
pub mod keyboard;
pub mod mouse;

struct Listener {
    keybd_hook_address: HHOOK,

    handle: JoinHandle<()>,
}

impl Listener {
    pub fn new() -> Self {
        let keybd_hook_address = install_hook(WH_KEYBOARD_LL, keybd_hook);
        Listener {
            keybd_hook_address,
            handle: Self::start_listening_thread(),
        }
    }

    fn start_listening_thread() -> JoinHandle<()> {
        thread::Builder::new()
            .name("win-lstn".into())
            .spawn(|| {})
            .unwrap()
    }
}

fn install_hook(
    hook_id: libc::c_int,
    hook_proc: unsafe extern "system" fn(libc::c_int, WPARAM, LPARAM) -> LRESULT,
) -> HHOOK {
    unsafe { SetWindowsHookExW(hook_id, Some(hook_proc), 0 as HINSTANCE, 0) }
}

unsafe extern "system" fn keybd_hook(
    code: libc::c_int,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    CallNextHookEx(null_mut(), code, w_param, l_param)
}
