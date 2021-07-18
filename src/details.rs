use crate::{install_hooks, process_message, Action, Event, Mouse, State};
use crate::{InhibitEvent, Keyboard};
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub(crate) fn registry() -> &'static Registry {
    lazy_static::lazy_static! {
        static ref REGISTRY: Registry = Registry::new();
    }
    &REGISTRY
}

struct Sequencer {
    _handle: JoinHandle<()>,

    tx: mpsc::Sender<Box<dyn Fn() + Send + Sync>>,
}

#[derive(Default)]
struct Pressed {
    pressed: Vec<Event>,
    pressed_keys: Vec<Keyboard>,
}

impl Pressed {
    pub(crate) fn is_pressed(&self, event: Event) -> bool {
        self.pressed.contains(&event)
    }

    // Order matters intentionally here.
    pub(crate) fn are_pressed(&self, keys: &[Keyboard]) -> bool {
        self.pressed_keys == keys
    }

    fn pressed(&mut self, event: Event) {
        if !self.pressed.contains(&event) {
            self.pressed.push(event);

            if let Event::Keyboard(key) = event {
                self.pressed_keys.push(key);
            }
        }
    }

    fn released(&mut self, event: Event) {
        if let Some(index) = self.pressed.iter().position(|e| *e == event) {
            self.pressed.remove(index);

            if let Event::Keyboard(key) = event {
                let pos = self
                    .pressed_keys
                    .iter()
                    .position(|k| *k == key)
                    .expect("state mismtch");
                self.pressed_keys.remove(pos);
            }
        }
    }
}

pub(crate) struct Registry {
    pub(crate) key_callbacks: Mutex<HashMap<Keyboard, Arc<Action>>>,
    pub(crate) button_callbacks: Mutex<HashMap<Mouse, Arc<Action>>>,
    pub(crate) any_key_callback: Mutex<Option<Arc<Action>>>,
    pub(crate) any_button_callback: Mutex<Option<Arc<Action>>>,
    #[allow(clippy::type_complexity)]
    pub(crate) hotkeys: Mutex<HashMap<Vec<Keyboard>, Arc<Box<dyn Fn() + Send + Sync + 'static>>>>,

    pressed: Mutex<Pressed>,

    _handle: JoinHandle<()>,
    sequencer: Mutex<Option<Sequencer>>,

    state: Mutex<HashMap<String, String>>,
}

impl Registry {
    pub(crate) fn new() -> Self {
        Registry {
            key_callbacks: Mutex::new(HashMap::new()),
            button_callbacks: Mutex::new(HashMap::new()),
            any_key_callback: Mutex::new(None),
            any_button_callback: Mutex::new(None),
            _handle: thread::Builder::new()
                .name("mki-lstn".into())
                .spawn(|| {
                    // For windows hooks need to be installed on the same thread that listens to the Messages.
                    install_hooks();
                    process_message();
                })
                .unwrap(),
            sequencer: Mutex::new(None),
            pressed: Mutex::new(Pressed::default()),
            hotkeys: Mutex::new(HashMap::new()),
            state: Mutex::new(HashMap::new()),
        }
    }

    pub(crate) fn sequence(&self, event: Event, state: State, action: Arc<Action>) {
        let erased_action = Box::new(move || {
            (action.callback)(event, state);
        });
        let mut sequencer = self.sequencer.lock().unwrap();
        let sequencer = sequencer.get_or_insert({
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

    fn invoke_action(&self, action: Arc<Action>, event: Event, state: State) {
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
                self.any_key_callback.lock().unwrap().clone(),
                self.key_callbacks.lock().unwrap().get(&key).cloned(),
            ),
            Event::Mouse(button) => (
                self.any_button_callback.lock().unwrap().clone(),
                self.button_callbacks.lock().unwrap().get(&button).cloned(),
            ),
        };
        (global_action, key_action)
    }

    pub(crate) fn event_down(&self, event: Event) -> InhibitEvent {
        self.pressed.lock().unwrap().pressed(event);
        let mut callbacks = Vec::new();
        if let Event::Keyboard(key) = event {
            for (sequence, callback) in self.hotkeys.lock().unwrap().iter() {
                if sequence.last() == Some(&key)
                    && self.pressed.lock().unwrap().are_pressed(&sequence)
                {
                    callbacks.push(callback.clone());
                }
            }
        }
        for callback in callbacks {
            // Should we not invoke actions if there is any hotkey present?
            thread::spawn(move || callback());
        }
        let state = State::Pressed;
        let mut inhibit = InhibitEvent::No;
        let (global_action, key_action) = self.map_event_to_actions(event);
        if let Some(action) = global_action {
            inhibit = action.inhibit.clone();
            self.invoke_action(action, event, state);
        }
        if let Some(action) = key_action {
            inhibit = action.inhibit.clone();
            self.invoke_action(action, event, state);
        }

        inhibit
    }

    pub(crate) fn event_up(&self, event: Event) -> InhibitEvent {
        self.pressed.lock().unwrap().released(event);
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

    #[cfg(target_os = "windows")] // Not sure how to detect double on linux
    pub(crate) fn event_click(&self, event: Event) -> InhibitEvent {
        let inhibit = self.event_down(event);
        self.event_up(event);
        inhibit
    }

    pub(crate) fn is_pressed(&self, event: Event) -> bool {
        self.pressed.lock().unwrap().is_pressed(event)
    }

    pub(crate) fn are_pressed(&self, keys: &[Keyboard]) -> bool {
        self.pressed.lock().unwrap().are_pressed(keys)
    }

    pub(crate) fn register_hotkey(
        &self,
        sequence: &[Keyboard],
        handler: impl Fn() + Send + Sync + 'static,
    ) {
        self.hotkeys
            .lock()
            .unwrap()
            .insert(sequence.to_vec(), Arc::new(Box::new(handler)));
    }

    pub(crate) fn unregister_hotkey(&self, sequence: &[Keyboard]) {
        self.hotkeys.lock().unwrap().remove(&sequence.to_vec());
    }

    //noinspection ALL
    pub fn set_state(&self, key: &str, value: &str) {
        self.state.lock().unwrap().insert(key.into(), value.into());
    }

    pub fn get_state(&self, key: &str) -> Option<String> {
        self.state.lock().unwrap().get(key).cloned()
    }
}
