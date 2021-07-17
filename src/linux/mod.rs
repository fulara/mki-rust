pub mod keyboard;

use crate::details::lock_registry;
use crate::keyboard::code_to_key;
use input::event::keyboard::{KeyState, KeyboardEventTrait};
use input::{Event, Libinput, LibinputInterface};
use nix::fcntl::{open, OFlag};
use nix::poll::{poll, PollFd, PollFlags};
use nix::sys::stat::Mode;
use nix::unistd::close;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use std::path::Path;

pub(crate) fn install_hooks() {}

pub(crate) fn process_message() {
    struct LibinputInterfaceRaw;

    impl LibinputInterface for LibinputInterfaceRaw {
        fn open_restricted(&mut self, path: &Path, flags: i32) -> std::result::Result<RawFd, i32> {
            if let Ok(fd) = open(path, OFlag::from_bits_truncate(flags), Mode::empty()) {
                Ok(fd)
            } else {
                Err(1)
            }
        }

        fn close_restricted(&mut self, fd: RawFd) {
            let _ = close(fd);
        }
    }
    let mut libinput = Libinput::new_with_udev(LibinputInterfaceRaw);
    libinput.udev_assign_seat(&"seat0").unwrap();
    let pollfd = PollFd::new(libinput.as_raw_fd(), PollFlags::POLLIN);
    while let Ok(_) = poll(&mut [pollfd], -1) {
        libinput.dispatch().unwrap();
        while let Some(event) = libinput.next() {
            handle_libinput_event(event);
        }
    }
}

fn handle_libinput_event(event: input::Event) {
    match event {
        Event::Device(_) => {}
        Event::Keyboard(kb) => {
            let key = code_to_key(kb.key());
            match kb.key_state() {
                KeyState::Pressed => {
                    lock_registry().key_down(key);
                }
                KeyState::Released => {
                    lock_registry().key_up(key);
                }
            }
        }
        Event::Pointer(_) => {}
        Event::Touch(_) => {}
        Event::Tablet(_) => {}
        Event::TabletPad(_) => {}
        Event::Gesture(_) => {}
        Event::Switch(_) => {}
    }
}
