use std::sync::mpsc;
use std::sync::mpsc::RecvTimeoutError;
use mki::{bind_key, Action, InhibitEvent, Keyboard, Sequence, register_channel, State, Event};
use std::thread;
use std::time::Duration;
use mki::details::Pressed;

fn maybe_send_event(maybe_event: Option<(Keyboard, State)>) {
    if let Some((event, state)) = maybe_event {
        match state {
            State::Pressed => {
                event.press();
            }
            State::Released => {
                event.release();
            }
        }
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    register_channel(tx);
    let mut last = None;
    let mut pressed = Pressed::default();
    let mut simultaneous = false;
    let mut active_ws = false;
    let mut active_yh = false;
    let mut combo_ws_key = Keyboard::LeftControl;
    let mut combo_yh_key = Keyboard::A;
    loop {
        match rx.recv_timeout(Duration::from_millis(1)) {
            Ok((event, state)) => {
                let mut a_combo = false;
                let mut combo_ws = false;
                let mut combo_yh = false;
                if state == State::Pressed {
                    pressed.pressed(Event::Keyboard(event));
                    if event == Keyboard::W && pressed.is_pressed(Event::Keyboard(Keyboard::S)) {
                        println!("simultaneous w+s");
                        a_combo = true;
                        combo_ws = true;
                        active_ws = true;
                    } else if event == Keyboard::S && pressed.is_pressed(Event::Keyboard(Keyboard::W)) {
                        println!("simultaneous s+w");
                        a_combo = true;
                        combo_ws = true;
                        active_ws = true;
                    }

                    if event == Keyboard::Y && pressed.is_pressed(Event::Keyboard(Keyboard::H)) {
                        println!("simultaneous yh");
                        a_combo = true;
                        combo_yh = true;
                        active_yh = true;
                    } else if event == Keyboard::H && pressed.is_pressed(Event::Keyboard(Keyboard::Y)) {
                        println!("simultaneous hy");
                        a_combo = true;
                        combo_yh = true;
                        active_yh = true;
                    }
                }
                if state == State::Released {
                    if active_ws && (event == Keyboard::W || event == Keyboard::S) {
                        combo_ws_key.release();
                        active_ws = false;
                        println!("releasing ws!")
                    }

                    if active_yh && (event == Keyboard::Y || event == Keyboard::H) {
                        combo_yh_key.release();
                        active_yh = false;
                        println!("releasing yh!")
                    }
                }
                if !a_combo {
                    maybe_send_event(last.take());
                    last = Some((event, state));
                } else {
                    last.take();
                }
                if combo_ws {
                    combo_ws_key.press();
                }
                if combo_yh {
                    combo_yh_key.press();
                }
            }
            Err(RecvTimeoutError::Timeout) => {
                pressed.clear();
                maybe_send_event(last.take());
                simultaneous = false;
            }
            Err(RecvTimeoutError::Disconnected) => {
                simultaneous = false;
                panic!();
            }
        }

    }
    thread::sleep(Duration::from_secs(100));
}
