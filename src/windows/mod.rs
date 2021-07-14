use crate::KeybdKey;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::convert::TryInto;
use std::mem::MaybeUninit;
use std::ptr::null_mut;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use winapi::shared::minwindef::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use winapi::shared::windef::HHOOK__;
use winapi::um::winuser::{
    CallNextHookEx, GetMessageW, SetWindowsHookExW, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL,
    WM_KEYDOWN,
};

pub mod hotkey;
pub mod keyboard;
pub mod mouse;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum InhibitEvent {
    Yes,
    No,
}

pub fn set_any_key_callback(callback: impl Fn(KeybdKey) -> InhibitEvent + Send + Sync + 'static) {
    *registry().lock().any_key_callback.lock() = Box::new(callback);
}

pub fn set_key_callback(
    key: KeybdKey,
    callback: impl Fn(KeybdKey) -> InhibitEvent + Send + Sync + 'static,
) {
    registry()
        .lock()
        .key_callbacks
        .lock()
        .insert(key, Box::new(callback));
}

lazy_static! {
    static ref REGISTRY: Arc<Mutex<Registry>> = Arc::new(Mutex::new(Registry::new()));
}

// Just because IDE does not bode well with lazy static.
fn registry() -> &'static Mutex<Registry> {
    &REGISTRY
}

struct Registry {
    key_callbacks:
        Arc<Mutex<HashMap<KeybdKey, Box<dyn Fn(KeybdKey) -> InhibitEvent + Send + Sync>>>>,
    any_key_callback: Arc<Mutex<Box<dyn Fn(KeybdKey) -> InhibitEvent + Send + Sync>>>,

    _handle: JoinHandle<()>,
}

impl Registry {
    fn new() -> Self {
        install_hook(WH_KEYBOARD_LL, keybd_hook);
        Registry {
            key_callbacks: Arc::new(Mutex::new(HashMap::new())),
            any_key_callback: Arc::new(Mutex::new(Box::new(|_| InhibitEvent::No))),
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
    let mut inhibit = InhibitEvent::No;
    // Note this seemingly is only activated when ALT is not pressed, need to handle WM_SYSKEYDOWN then
    // Test that case.
    if w_param as u32 == WM_KEYDOWN {
        let vk: u16 = (*(l_param as *const KBDLLHOOKSTRUCT))
            .vkCode
            .try_into()
            .expect("vkCode does not fit in u16");
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
        let listener = registry().lock();
        inhibit = listener.any_key_callback.lock()(key);
        let key_callbacks = listener.key_callbacks.lock();
        if let Some(callback) = key_callbacks.get(&key) {
            inhibit = callback(key)
        }
    }

    if inhibit == InhibitEvent::Yes {
        1
    } else {
        CallNextHookEx(null_mut(), code, w_param, l_param)
    }
}
