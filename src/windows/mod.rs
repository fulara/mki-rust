use std::mem::MaybeUninit;
use std::ptr::null_mut;
use std::thread;
use std::thread::JoinHandle;
use winapi::shared::minwindef::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use winapi::shared::windef::HHOOK__;
use winapi::um::winuser::{CallNextHookEx, GetMessageW, SetWindowsHookExW, MSG, WH_KEYBOARD_LL};

pub mod hotkey;
pub mod keyboard;
pub mod mouse;

pub struct Listener {
    _keybd_hook_address: *mut HHOOK__,

    _handle: JoinHandle<()>,
}

impl Listener {
    pub fn new(_callback: impl Fn() -> bool + Send + Sync) -> Self {
        let keybd_hook_address = install_hook(WH_KEYBOARD_LL, keybd_hook);
        Listener {
            _keybd_hook_address: keybd_hook_address,
            _handle: Self::start_listening_thread(),
        }
    }

    fn start_listening_thread() -> JoinHandle<()> {
        thread::Builder::new()
            .name("win-lstn".into())
            .spawn(|| loop {
                let mut msg: MSG = unsafe { MaybeUninit::zeroed().assume_init() };
                unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
                println!("Received message in GetMessageW")
            })
            .unwrap()
    }
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
    CallNextHookEx(null_mut(), code, w_param, l_param)
}
