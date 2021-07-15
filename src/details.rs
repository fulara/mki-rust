use crate::{install_hooks, process_message};
use crate::{InhibitEvent, KeybdKey};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::thread::JoinHandle;

pub(crate) fn lock_registry() -> MutexGuard<'static, Registry> {
    lazy_static::lazy_static! {
        static ref REGISTRY: Arc<Mutex<Registry>> = Arc::new(Mutex::new(Registry::new()));
    }
    REGISTRY.lock().unwrap()
}

pub(crate) struct Registry {
    pub(crate) key_callbacks:
        HashMap<KeybdKey, Box<dyn Fn(KeybdKey) -> InhibitEvent + Send + Sync>>,
    pub(crate) any_key_callback: Box<dyn Fn(KeybdKey) -> InhibitEvent + Send + Sync>,

    _handle: JoinHandle<()>,
}

impl Registry {
    pub(crate) fn new() -> Self {
        Registry {
            key_callbacks: HashMap::new(),
            any_key_callback: Box::new(|_| InhibitEvent::No),
            _handle: thread::Builder::new()
                .name("mki-lstn".into())
                .spawn(|| {
                    // For windows hooks need to be installed on the same thread that listens to the Messages.
                    install_hooks();
                    process_message();
                })
                .unwrap(),
        }
    }
}
