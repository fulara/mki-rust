use crate::install_hooks;
use crate::start_listening_thread;
use crate::{InhibitEvent, KeybdKey};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
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
        Arc<Mutex<HashMap<KeybdKey, Box<dyn Fn(KeybdKey) -> InhibitEvent + Send + Sync>>>>,
    pub(crate) any_key_callback: Arc<Mutex<Box<dyn Fn(KeybdKey) -> InhibitEvent + Send + Sync>>>,

    _handle: JoinHandle<()>,
}

impl Registry {
    pub(crate) fn new() -> Self {
        install_hooks();
        Registry {
            key_callbacks: Arc::new(Mutex::new(HashMap::new())),
            any_key_callback: Arc::new(Mutex::new(Box::new(|_| InhibitEvent::No))),
            _handle: start_listening_thread(),
        }
    }
}
