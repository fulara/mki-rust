use crate::{install_hooks, process_message};
use crate::{InhibitEvent, KeybdKey};
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex, MutexGuard};
use std::thread;
use std::thread::JoinHandle;

pub(crate) fn lock_registry() -> MutexGuard<'static, Registry> {
    lazy_static::lazy_static! {
        static ref REGISTRY: Arc<Mutex<Registry>> = Arc::new(Mutex::new(Registry::new()));
    }
    REGISTRY.lock().unwrap()
}

struct Sequencer {
    _handle: JoinHandle<()>,

    tx: mpsc::Sender<Box<dyn Fn() + Send + Sync>>,
}

pub(crate) struct Registry {
    pub(crate) key_callbacks:
        HashMap<KeybdKey, Box<dyn Fn(KeybdKey) -> InhibitEvent + Send + Sync>>,
    pub(crate) any_key_callback: Box<dyn Fn(KeybdKey) -> InhibitEvent + Send + Sync>,

    _handle: JoinHandle<()>,
    sequencer: Option<Sequencer>,
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
            sequencer: None,
        }
    }

    pub(crate) fn sequence(
        &mut self,
        key: KeybdKey,
        action: impl Fn(KeybdKey) + Clone + Send + Sync + 'static,
    ) {
        let erased_action = Box::new(move || {
            action(key);
        });
        let sequencer = self.sequencer.get_or_insert({
            let (tx, rx) = mpsc::channel::<Box<dyn Fn() + Send + Sync>>();
            thread::Builder::new();
            Sequencer {
                _handle: thread::Builder::new()
                    .name("sequencer".into())
                    .spawn(move || {
                        while let Ok(action) = rx.recv() {
                            action()
                        }
                    })
                    .unwrap(),
                tx,
            }
        });
        let _ = sequencer.tx.send(erased_action);
    }

    pub(crate) fn key_down(&self, key: KeybdKey) {
        (self.any_key_callback)(key);
        if let Some(mapping) = self.key_callbacks.get(&key) {
            mapping(key);
        }
    }

    pub(crate) fn key_up(&self, _key: KeybdKey) {
        // TODO: I guess?
    }
}
