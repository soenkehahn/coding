use crate::programs;
use crate::utils;
use cradle::prelude::*;
use std::thread::sleep;
use std::time::Duration;

pub fn setup(vcs_type: crate::VcsType) {
    swaymsg("layout tabbed");
    wait_for_change(
        "firefox",
        "exec firefox --new-window github.com/notifications",
    );
    swaymsg("move left");
    swaymsg("split toggle");
    swaymsg("layout tabbed");
    swaymsg("focus right");
    // right shell for git
    wait_for_change("terminal", terminal_command(None));
    swaymsg("split toggle");
    swaymsg("layout splith");
    wait_for_change("vcs tree", terminal_command(programs::tree(vcs_type)));
    swaymsg("split toggle");
    swaymsg("layout splitv");
    wait_for_change("vcs status", terminal_command(programs::status(vcs_type)));
    swaymsg("focus left");
    swaymsg("focus left");
    // middle tab
    swaymsg("split toggle");
    swaymsg("layout splith");
    // shell top right
    wait_for_change("terminal", terminal_command(None));
    swaymsg("split toggle");
    swaymsg("layout splitv");
    swaymsg("focus left");
    swaymsg("split toggle");
    swaymsg("layout stacking");
    swaymsg("resize set 60 ppt 0");
}

fn swaymsg(sway_command: impl Input) {
    let StdoutTrimmed(result) = run_output!("swaymsg", sway_command);
    let expected = "[\n  {\n    \"success\": true\n  }\n]";
    if result != expected {
        utils::abort(&format!("{:?} != {:?}", result, expected));
    }
}

fn wait_for_change(message: &str, sway_command: impl Input) {
    let old = get_active_window();
    swaymsg(sway_command);
    eprintln!("waiting for {}...", message);
    utils::wait_for(|| {
        let new = get_active_window();
        new != old
    });
    sleep(Duration::from_secs_f32(0.1));
}

fn get_active_window() -> String {
    let StdoutUntrimmed(tree) = run_output!(%"swaymsg -t get_tree");
    let StdoutTrimmed(id) = run_output!(
        Stdin(tree),
        "jq",
        ".. | (.nodes? // empty)[] | select(.focused==true) | .id"
    );
    id
}

fn terminal_command(command: Option<Vec<String>>) -> impl Input {
    ("exec", programs::terminal_command(command))
}
