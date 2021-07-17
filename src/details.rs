use crate::{install_hooks, process_message, Action, KeyState};
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
    pub(crate) key_callbacks: HashMap<KeybdKey, Arc<Action>>,
    pub(crate) any_key_callback: Option<Arc<Action>>,

    _handle: JoinHandle<()>,
    sequencer: Option<Sequencer>,
}

impl Registry {
    pub(crate) fn new() -> Self {
        Registry {
            key_callbacks: HashMap::new(),
            any_key_callback: None,
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

    pub(crate) fn sequence(&mut self, key: KeybdKey, state: KeyState, action: Arc<Action>) {
        let erased_action = Box::new(move || {
            (action.callback)(key, state);
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

    fn invoke_action(&mut self, action: Arc<Action>, key: KeybdKey, state: KeyState) {
        if action.defer {
            thread::spawn(move || {
                (action.callback)(key, state);
            });
        } else if action.sequencer {
            self.sequence(key, state, action)
        } else {
            (action.callback)(key, state);
        }
    }

    pub(crate) fn key_down(&mut self, key: KeybdKey) -> InhibitEvent {
        let state = KeyState::Pressed;
        let mut inhibit = InhibitEvent::No;
        if let Some(action) = self.any_key_callback.clone() {
            inhibit = action.inhibit;
            self.invoke_action(action, key, state);
        }
        if let Some(action) = self.key_callbacks.get(&key).cloned() {
            inhibit = action.inhibit;
            self.invoke_action(action, key, state);
        }
        inhibit
    }

    pub(crate) fn key_up(&mut self, key: KeybdKey) -> InhibitEvent {
        let state = KeyState::Released;
        if let Some(action) = self.any_key_callback.clone() {
            self.invoke_action(action, key, state);
        }
        if let Some(action) = self.key_callbacks.get(&key).cloned() {
            self.invoke_action(action, key, state);
        }
        InhibitEvent::No
    }
}
