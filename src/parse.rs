use crate::{Keyboard, Mouse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Config {
    bind: Vec<Bind>,
}

#[derive(Deserialize, Serialize)]
struct Bind {
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
    Press(Input),
    Release(Input),
    Click(Input),
    Sleep(i64), // Milliseconds
    SetState(SetState),
    Println(String),
    PrintState(String),
}

#[cfg(test)]
mod tests {
    use crate::parse::{Action, Bind, Config, Input, Pressed, SetState, StateMatches};
    use crate::Keyboard::{LeftControl, Number0, Number1, D, E, H, Q, R, S, W};

    #[test]
    fn example() {
        let c = Config {
            bind: vec![
                Bind {
                    input: Input {
                        key: Some(vec![LeftControl, H]),
                        button: None,
                    },
                    action: Action::Multi(vec![
                        Action::Pressed(Pressed {
                            input: Input {
                                key: Some(vec![Q]),
                                button: None,
                            },
                            action: vec![Action::Click(Input::key(W)), Action::Sleep(10)],
                        }),
                        Action::Click(Input::key(E)),
                    ]),
                },
                Bind {
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
  - key:
      - LeftControl
      - H
    action:
      multi:
        - pressed:
            input:
              key:
                - Q
            action:
              - click:
                  key:
                    - W
              - sleep: 10
        - click:
            key:
              - E
  - key:
      - S
    action:
      multi:
        - set-state:
            name: test
            value: "1"
        - print-state: test
  - key:
      - R
    action:
      multi:
        - set-state:
            name: test
            value: "0"
        - print-state: test
  - key:
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
}
