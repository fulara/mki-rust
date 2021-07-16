pub mod keyboard;

use std::thread;
use std::thread::JoinHandle;

pub(crate) fn install_hooks() {}

pub(crate) fn start_listening_thread() -> JoinHandle<()> {
    thread::Builder::new()
        .name("lin-lstn".into())
        .spawn(|| loop {})
        .unwrap()
}

pub(crate) fn process_message() {
    ::std::thread::sleep_ms(1000); // todo.
}
