use mki::load_config;
use std::thread;
use std::time::Duration;

fn main() {
    let cfg = r#"---
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
"#;
    load_config(cfg).unwrap();
    thread::sleep(Duration::from_secs(1000));
}
