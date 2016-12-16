extern crate libc;

pub const MODIFIERS_NONE: i32 = 0;
pub const MODIFIERS_ALT: i32 = 1;
pub const MODIFIERS_CTRL: i32 = 2;
pub const MODIFIERS_SHIFT: i32 = 4;
pub const MODIFIERS_WIN: i32 = 8;

extern {
    fn register_hotkey_c(key_code: libc::c_ushort, modifiers: libc::c_int, id: libc::c_int) -> libc::c_int;
    fn wait_for_hotkey_c() -> libc::c_int;
}

pub fn register_hotkey(key_code: u16, modifiers : i32, hotkey_id : i32) {
    unsafe { register_hotkey_c(key_code, modifiers, hotkey_id); }
}

pub fn wait_for_hotkey() -> i32 {
    unsafe { wait_for_hotkey_c() }
}
