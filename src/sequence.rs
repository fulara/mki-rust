use crate::Keyboard;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
/// A sequence of events to execute.
pub struct Sequence {
    sequence: Vec<Vec<Keyboard>>,
}

impl Sequence {
    /// A Sequence of events to execute parsed from some text.
    /// Can only be created if all the keys are supported. upon encountering a key that cannot be
    /// represented will return None.
    pub fn text(text: &str) -> Option<Self> {
        let mut sequence = Vec::new();
        for char in text.chars() {
            let uppercase = char.to_ascii_uppercase();
            use Keyboard::*;
            let key = Keyboard::from_str(&uppercase.to_string()).ok()?;
            if char.is_uppercase() || char == ':' || char == '\"' {
                sequence.push(vec![LeftShift, key])
            } else {
                sequence.push(vec![key])
            }
        }
        Some(Sequence { sequence })
    }

    /// send this Sequence on new thread.
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