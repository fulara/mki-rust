use crate::{Key, KeybdKey};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Sequence {
    sequence: Vec<Vec<KeybdKey>>,
}

impl Sequence {
    pub fn text(text: &str) -> Option<Self> {
        let mut sequence = Vec::new();
        for char in text.chars() {
            let lowercase = char.to_ascii_lowercase();
            use KeybdKey::*;
            let key = match lowercase {
                ' ' => Space,
                'a' => A,
                'b' => B,
                'c' => C,
                'd' => D,
                'e' => E,
                'f' => F,
                'g' => G,
                'h' => H,
                'i' => I,
                'j' => J,
                'k' => K,
                'l' => L,
                'm' => M,
                'n' => N,
                'o' => O,
                'p' => P,
                'q' => Q,
                'r' => R,
                's' => S,
                't' => T,
                'u' => U,
                'v' => V,
                'w' => W,
                'x' => X,
                'y' => Y,
                'z' => Z,
                _ => {
                    return None;
                }
            };
            if char.is_uppercase() {
                sequence.push(vec![LeftShift, key])
            } else {
                sequence.push(vec![key])
            }
        }
        return Some(Sequence { sequence });
    }

    pub fn send(&self) {
        let cloned = self.clone();
        thread::spawn(move || {
            for keys in &cloned.sequence {
                for key in keys {
                    key.press();
                }
                thread::sleep(Duration::from_millis(15));
                for key in keys {
                    key.release();
                }
                thread::sleep(Duration::from_millis(15));
            }
        });
    }
}
