use crate::KeybdKey;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;
use uinput::event::keyboard::Key;
use uinput::event::relative::Position;

enum KeybdAction {
    Press,
    Release,
    Click,
}

impl crate::Key for KeybdKey {
    fn press(&self) {
        send_key_stroke(KeybdAction::Press, *self);
    }

    fn release(&self) {
        send_key_stroke(KeybdAction::Release, *self);
    }

    fn click(&self) {
        send_key_stroke(KeybdAction::Click, *self);
    }

    fn is_pressed(&self) -> bool {
        todo!()
    }

    fn is_toggled(&self) -> bool {
        todo!()
    }
}

fn send_key_stroke(action: KeybdAction, key: KeybdKey) {
    let mut device = device();
    if let Some(key) = key_to_event(key) {
        match action {
            KeybdAction::Press => device.press(&key).unwrap(),
            KeybdAction::Release => device.release(&key).unwrap(),
            KeybdAction::Click => device.click(&key).unwrap(),
        }
    }
    device.synchronize().unwrap();
}

fn device() -> MutexGuard<'static, uinput::Device> {
    lazy_static::lazy_static! {
        static ref DEVICE: Arc<Mutex<uinput::Device>> = {
            let device = Arc::new(Mutex::new(
                uinput::default()
                .unwrap()
                .name("mki")
                .unwrap()
                .event(uinput::event::Keyboard::All)
                .unwrap()
                .event(Position::X)
                .unwrap()
                .event(Position::Y)
                .unwrap()
                .create()
                .unwrap()));
            // Without this there seems to be some inputs gone to hell
            device.lock().unwrap().synchronize().unwrap();
            std::thread::sleep(Duration::from_millis(100));
            device
        };
    }
    DEVICE.lock().unwrap()
}

pub fn key_to_event(key: KeybdKey) -> Option<Key> {
    use KeybdKey::*;
    match key {
        Backspace => Some(Key::BackSpace),
        Tab => Some(Key::Tab),
        Enter => Some(Key::Enter),
        Escape => Some(Key::Esc),
        Space => Some(Key::Space),
        Home => Some(Key::Home),
        Left => Some(Key::Left),
        Up => Some(Key::Up),
        Right => Some(Key::Right),
        Down => Some(Key::Down),
        Insert => Some(Key::Insert),
        Delete => Some(Key::Delete),
        Numrow0 => Some(Key::_0),
        Numrow1 => Some(Key::_1),
        Numrow2 => Some(Key::_2),
        Numrow3 => Some(Key::_3),
        Numrow4 => Some(Key::_4),
        Numrow5 => Some(Key::_5),
        Numrow6 => Some(Key::_6),
        Numrow7 => Some(Key::_7),
        Numrow8 => Some(Key::_8),
        Numrow9 => Some(Key::_9),
        A => Some(Key::A),
        B => Some(Key::B),
        C => Some(Key::C),
        D => Some(Key::D),
        E => Some(Key::E),
        F => Some(Key::F),
        G => Some(Key::G),
        H => Some(Key::H),
        I => Some(Key::I),
        J => Some(Key::J),
        K => Some(Key::K),
        L => Some(Key::L),
        M => Some(Key::M),
        N => Some(Key::N),
        O => Some(Key::O),
        P => Some(Key::P),
        Q => Some(Key::Q),
        R => Some(Key::R),
        S => Some(Key::S),
        T => Some(Key::T),
        U => Some(Key::U),
        V => Some(Key::V),
        W => Some(Key::W),
        X => Some(Key::X),
        Y => Some(Key::Y),
        Z => Some(Key::Z),
        Numpad0 => Some(Key::_0),
        Numpad1 => Some(Key::_1),
        Numpad2 => Some(Key::_2),
        Numpad3 => Some(Key::_3),
        Numpad4 => Some(Key::_4),
        Numpad5 => Some(Key::_5),
        Numpad6 => Some(Key::_6),
        Numpad7 => Some(Key::_7),
        Numpad8 => Some(Key::_8),
        Numpad9 => Some(Key::_9),
        F1 => Some(Key::F1),
        F2 => Some(Key::F2),
        F3 => Some(Key::F3),
        F4 => Some(Key::F4),
        F5 => Some(Key::F5),
        F6 => Some(Key::F6),
        F7 => Some(Key::F7),
        F8 => Some(Key::F8),
        F9 => Some(Key::F9),
        F10 => Some(Key::F10),
        NumLock => Some(Key::NumLock),
        ScrollLock => Some(Key::ScrollLock),
        CapsLock => Some(Key::CapsLock),
        LShift => Some(Key::LeftShift),
        RShift => Some(Key::RightShift),
        LControl => Some(Key::LeftControl),
        F11 => Some(Key::F11),
        F12 => Some(Key::F12),
        F13 => Some(Key::F13),
        F14 => Some(Key::F14),
        F15 => Some(Key::F15),
        F16 => Some(Key::F16),
        F17 => Some(Key::F17),
        F18 => Some(Key::F18),
        F19 => Some(Key::F19),
        F20 => Some(Key::F20),
        F21 => Some(Key::F21),
        F22 => Some(Key::F22),
        F23 => Some(Key::F23),
        F24 => Some(Key::F24),
        RControl => Some(Key::RightControl),
        Other(_code) => None,
    }
}
