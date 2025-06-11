use crate::programs;
use crate::utils;
use cradle::prelude::*;
use std::thread::sleep;
use std::time::Duration;

pub fn setup() {
    // editor terminal
    action(&["set-column-width", "70%"]);
    // browser
    wait_for_change("firefox", || {
        run("firefox --new-window github.com/notifications".split_whitespace());
    });
    action(&["move-column-left"]);
    action(&["set-column-width", "90%"]);
    action(&["focus-column-right"]);
    // watcher terminal
    wait_for_change("terminal", || run(programs::terminal_command(None)));
    action(&["set-column-width", "40%"]);
    // vcs terminals
    wait_for_change("terminal", || {
        run(programs::terminal_command(Some(vec![
            "git-shell".to_string(),
        ])));
    });
    action(&["set-column-width", "100%"]);
    // go back
    action(&["focus-column-left"]);
    action(&["focus-column-left"]);
}

fn run(command: impl Input) {
    ("niri", "msg", "action", "spawn", "--", command).run();
}

fn action(action: &[&str]) {
    ("niri", "msg", "action", action).run();
}

fn wait_for_change(message: &str, action: impl FnOnce()) {
    let old = get_active_window();
    action();
    eprintln!("waiting for {}...", message);
    utils::wait_for(|| {
        let new = get_active_window();
        new != old
    });
    sleep(Duration::from_secs_f32(0.1));
}

fn get_active_window() -> String {
    let StdoutUntrimmed(tree) = "niri msg --json windows".split_whitespace().run_output();
    let StdoutTrimmed(id) = run_output!(Stdin(tree), "jq", ".[] | select(.is_focused==true) | .id");
    id
}
