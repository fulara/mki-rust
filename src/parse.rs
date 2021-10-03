use crate::{are_pressed, get_state, register_hotkey, set_state, Keyboard, Mouse};
use serde::de::Error;
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;

#[derive(Deserialize, Serialize)]
struct Config {
    bind: Vec<Bind>,
}

#[derive(Deserialize, Serialize)]
struct Bind {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(flatten)]
    input: Input,

    action: Action,
}

#[derive(Deserialize, Serialize)]
struct Input {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<Vec<Keyboard>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    button: Option<Vec<Mouse>>,
}

impl Input {
    fn validate(&self) -> Result<(), serde_yaml::Error> {
        if self.key.is_none() && self.button.is_none() {
            Err(serde_yaml::Error::custom("Bind had neither key nor button"))
        } else if self.key.is_some() && self.button.is_some() {
            Err(serde_yaml::Error::custom("Bind had both key and button"))
        } else if let Some(keys) = self.key.as_ref() {
            if keys.is_empty() {
                Err(serde_yaml::Error::custom("Bind had empty keys"))
            } else {
                Ok(())
            }
        } else if let Some(buttons) = self.button.as_ref() {
            if buttons.is_empty() {
                Err(serde_yaml::Error::custom("Bind had empty buttons"))
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
    #[allow(unused)]
    fn key(key: Keyboard) -> Self {
        Input {
            button: None,
            key: Some(vec![key]),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Pressed {
    input: Input,
    action: Vec<Action>,
}

#[derive(Deserialize, Serialize)]
struct SetState {
    name: String,
    value: String,
}

#[derive(Deserialize, Serialize)]
struct StateMatches {
    name: String,
    value: String,
    action: Vec<Action>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Action {
    Multi(Vec<Action>),
    Pressed(Pressed),
    StateMatches(StateMatches),
    WhileStateMatches(StateMatches),
    Press(Input),
    Release(Input),
    Click(Input),
    Sleep(u64), // Milliseconds
    SetState(SetState),
    Println(String),
    PrintState(String),
}

fn validate_actions(actions: &[Action]) -> serde_yaml::Result<()> {
    for a in actions {
        validate_action(a)?;
    }
    Ok(())
}

fn validate_action(action: &Action) -> serde_yaml::Result<()> {
    match action {
        Action::Multi(actions) => {
            validate_actions(actions)?;
        }
        Action::Pressed(pressed) => {
            pressed.input.validate()?;
            if pressed.input.key.is_none() {
                return Err(serde_yaml::Error::custom("Pressed can only check keys."));
            }
            validate_actions(&pressed.action)?;
        }
        Action::StateMatches(state_matches) => {
            validate_actions(&state_matches.action)?;
        }
        Action::WhileStateMatches(state_matches) => {
            validate_actions(&state_matches.action)?;
        }
        Action::Press(_)
        | Action::Release(_)
        | Action::Click(_)
        | Action::Sleep(_)
        | Action::SetState(_)
        | Action::Println(_)
        | Action::PrintState(_) => {}
    }

    Ok(())
}

fn handle_actions(actions: &[Action]) {
    for a in actions {
        handle_action(a);
    }
}

fn handle_action(action: &Action) {
    match action {
        Action::Multi(actions) => {
            handle_actions(actions);
        }
        Action::Pressed(pressed) => {
            let keys = pressed.input.key.as_ref().unwrap();
            if are_pressed(keys) {
                handle_actions(&pressed.action);
            }
        }
        Action::StateMatches(state_matches) => {
            if let Some(state) = get_state(&state_matches.name) {
                if state == state_matches.value {
                    handle_actions(&state_matches.action);
                }
            }
        }
        Action::WhileStateMatches(state_matches) => {
            while let Some(state) = get_state(&state_matches.name) {
                if state == state_matches.value {
                    handle_actions(&state_matches.action);
                } else {
                    break;
                }
            }
        }
        Action::Press(input) => {
            if let Some(keys) = &input.key {
                for k in keys {
                    k.press();
                }
            }
            if let Some(buttons) = &input.button {
                for b in buttons {
                    b.press();
                }
            }
        }
        Action::Release(input) => {
            if let Some(keys) = &input.key {
                for k in keys {
                    k.release();
                }
            }
            if let Some(buttons) = &input.button {
                for b in buttons {
                    b.release();
                }
            }
        }
        Action::Click(input) => {
            if let Some(keys) = &input.key {
                for k in keys {
                    k.click()
                }
            }
            if let Some(buttons) = &input.button {
                for b in buttons {
                    b.click()
                }
            }
        }
        Action::Sleep(millis) => {
            thread::sleep(Duration::from_millis(*millis));
        }
        Action::SetState(state) => set_state(&state.name, &state.value),
        Action::Println(message) => {
            println!("{}", message);
        }
        Action::PrintState(state) => {
            println!("State under key: {} is: {:?}", state, get_state(state))
        }
    }
}

pub fn load_config(content: &str) -> Result<(), serde_yaml::Error> {
    let config: Config = serde_yaml::from_str(content)?;
    for bind in config.bind {
        bind.input.validate()?;
        let keys = bind.input.key.unwrap();
        println!("Now binding a hotkey for: {:?}", keys);
        if let Some(description) = bind.description {
            println!("description: {}", description);
        }
        let action = bind.action;
        validate_action(&action)?;
        register_hotkey(&keys, move || {
            handle_action(&action);
        })
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse::{Action, Bind, Config, Input, SetState, StateMatches};
    use crate::Keyboard::{LeftControl, Number0, Number1, D, E, H, K, L, R, S, W};

    #[test]
    fn example() {
        let c = Config {
            bind: vec![
                Bind {
                    description: Some(
                        "LCtrl + H: [Loop until state is 1 [printing W, Sleep100]], then print E"
                            .into(),
                    ),
                    input: Input {
                        key: Some(vec![LeftControl, H]),
                        button: None,
                    },
                    action: Action::Multi(vec![
                        Action::WhileStateMatches(StateMatches {
                            name: "test".into(),
                            value: "1".into(),
                            action: vec![Action::Click(Input::key(W)), Action::Sleep(100)],
                        }),
                        Action::Click(Input::key(E)),
                    ]),
                },
                Bind {
                    description: Some("S: Set state to 1 then print it".into()),
                    input: Input::key(S),
                    action: Action::Multi(vec![
                        Action::SetState(SetState {
                            name: "test".into(),
                            value: "1".into(),
                        }),
                        Action::PrintState("test".into()),
                    ]),
                },
                Bind {
                    description: Some("R: Set state to 0 then print it".into()),
                    input: Input::key(R),
                    action: Action::Multi(vec![
                        Action::SetState(SetState {
                            name: "test".into(),
                            value: "0".into(),
                        }),
                        Action::PrintState("test".into()),
                    ]),
                },
                Bind {
                    description: Some("If state 1 then click 1; If state 0 then click 0".into()),
                    input: Input::key(D),
                    action: Action::Multi(vec![
                        Action::StateMatches(StateMatches {
                            name: "test".into(),
                            value: "1".into(),
                            action: vec![Action::Click(Input::key(Number1))],
                        }),
                        Action::StateMatches(StateMatches {
                            name: "test".into(),
                            value: "0".into(),
                            action: vec![Action::Click(Input::key(Number0))],
                        }),
                    ]),
                },
            ],
        };
        assert_eq!(
            r#"---
bind:
  - description: "LCtrl + H: [Loop until state is 1 [printing W, Sleep100]], then print E"
    key:
      - LeftControl
      - H
    action:
      multi:
        - while-state-matches:
            name: test
            value: "1"
            action:
              - click:
                  key:
                    - W
              - sleep: 100
        - click:
            key:
              - E
  - description: "S: Set state to 1 then print it"
    key:
      - S
    action:
      multi:
        - set-state:
            name: test
            value: "1"
        - print-state: test
  - description: "R: Set state to 0 then print it"
    key:
      - R
    action:
      multi:
        - set-state:
            name: test
            value: "0"
        - print-state: test
  - description: If state 1 then click 1; If state 0 then click 0
    key:
      - D
    action:
      multi:
        - state-matches:
            name: test
            value: "1"
            action:
              - click:
                  key:
                    - Number1
        - state-matches:
            name: test
            value: "0"
            action:
              - click:
                  key:
                    - Number0
"#,
            serde_yaml::to_string(&c).unwrap()
        )
    }

    #[test]
    fn readme_example() {
        let c = Config {
            bind: vec![Bind {
                description: Some("Whenever Ctrl+L is clicked click K as well".into()),
                input: Input {
                    key: Some(vec![LeftControl, L]),
                    button: None,
                },
                action: Action::Click(Input::key(K)),
            }],
        };
        assert_eq!(
            r#"---
bind:
  - description: Whenever Ctrl+L is clicked click K as well
    key:
      - LeftControl
      - L
    action:
      click:
        key:
          - K
"#,
            serde_yaml::to_string(&c).unwrap()
        );
    }
}
