use crate::install_hooks;
use crate::start_listening_thread;
use crate::{InhibitEvent, KeybdKey};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

lazy_static::lazy_static! {
    pub(crate) static ref REGISTRY: Arc<Mutex<Registry>> = Arc::new(Mutex::new(Registry::new()));
}

// Just because IDE does not bode well with lazy static.
pub(crate) fn registry() -> &'static Mutex<Registry> {
    &REGISTRY
}

pub(crate) struct Registry {
    pub(crate) key_callbacks:
        HashMap<KeybdKey, Box<dyn Fn(KeybdKey) -> InhibitEvent + Send + Sync>>,
    pub(crate) any_key_callback: Box<dyn Fn(KeybdKey) -> InhibitEvent + Send + Sync>,

    _handle: JoinHandle<()>,
}

impl Registry {
    pub(crate) fn new() -> Self {
        install_hooks();
        Registry {
            key_callbacks: HashMap::new(),
            any_key_callback: Box::new(|_| InhibitEvent::No),
            _handle: start_listening_thread(),
        }
    }
}
