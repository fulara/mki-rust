use std::thread;
use std::thread::JoinHandle;

pub mod keyboard;

pub(crate) fn install_hooks() {}

pub(crate) fn start_listening_thread() -> JoinHandle<()> {
    thread::Builder::new()
        .name("lin-lstn".into())
        .spawn(|| loop {})
        .unwrap()
}