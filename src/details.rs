use crate::{install_hooks, process_message, Action, Event, MouseButton, State};
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
    pub(crate) button_callbacks: HashMap<MouseButton, Arc<Action>>,
    pub(crate) any_key_callback: Option<Arc<Action>>,
    pub(crate) any_button_callback: Option<Arc<Action>>,

    pub(crate) pressed: Vec<Event>,

    _handle: JoinHandle<()>,
    sequencer: Option<Sequencer>,
}

impl Registry {
    pub(crate) fn new() -> Self {
        Registry {
            key_callbacks: HashMap::new(),
            button_callbacks: HashMap::new(),
            any_key_callback: None,
            any_button_callback: None,
            _handle: thread::Builder::new()
                .name("mki-lstn".into())
                .spawn(|| {
                    // For windows hooks need to be installed on the same thread that listens to the Messages.
                    install_hooks();
                    process_message();
                })
                .unwrap(),
            sequencer: None,
            pressed: Vec::new(),
        }
    }

    pub(crate) fn sequence(&mut self, event: Event, state: State, action: Arc<Action>) {
        let erased_action = Box::new(move || {
            (action.callback)(event, state);
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

    fn invoke_action(&mut self, action: Arc<Action>, event: Event, state: State) {
        if action.defer {
            thread::spawn(move || {
                (action.callback)(event, state);
            });
        } else if action.sequencer {
            self.sequence(event, state, action)
        } else {
            (action.callback)(event, state);
        }
    }

    fn map_event_to_actions(&self, event: Event) -> (Option<Arc<Action>>, Option<Arc<Action>>) {
        let (global_action, key_action) = match event {
            Event::Keyboard(key) => (
                self.any_key_callback.clone(),
                self.key_callbacks.get(&key).cloned(),
            ),
            Event::Mouse(button) => (
                self.any_button_callback.clone(),
                self.button_callbacks.get(&button).cloned(),
            ),
        };
        (global_action, key_action)
    }

    pub(crate) fn event_down(&mut self, event: Event) -> InhibitEvent {
        self.pressed(event);
        let state = State::Pressed;
        let mut inhibit = InhibitEvent::No;
        let (global_action, key_action) = self.map_event_to_actions(event);
        if let Some(action) = global_action {
            inhibit = action.inhibit;
            self.invoke_action(action, event, state);
        }
        if let Some(action) = key_action {
            inhibit = action.inhibit;
            self.invoke_action(action, event, state);
        }

        inhibit
    }

    pub(crate) fn event_up(&mut self, event: Event) -> InhibitEvent {
        self.released(event);
        let state = State::Released;
        let (global_action, key_action) = self.map_event_to_actions(event);
        if let Some(action) = global_action {
            self.invoke_action(action, event, state);
        }
        if let Some(action) = key_action {
            self.invoke_action(action, event, state);
        }

        InhibitEvent::No
    }

    pub(crate) fn is_pressed(&self, event: Event) -> bool {
        self.pressed.contains(&event)
    }

    // Order matters intentionally here.
    pub(crate) fn are_pressed(&self, events: &[Event]) -> bool {
        let mut iter = events.iter();
        let mut searched = if let Some(searched) = iter.next() {
            searched
        } else {
            // are_pressed invoked with empty events? sure.
            return true;
        };
        for e in &self.pressed {
            if searched == e {
                searched = if let Some(searched) = iter.next() {
                    searched
                } else {
                    return true;
                };
            }
        }

        false
    }

    fn pressed(&mut self, event: Event) {
        if !self.pressed.contains(&event) {
            self.pressed.push(event);
        }
    }

    fn released(&mut self, event: Event) {
        if let Some(index) = self.pressed.iter().position(|e| *e == event) {
            self.pressed.remove(index);
        }
    }
}
