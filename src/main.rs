#!/usr/bin/env rust-script

use cradle::prelude::*;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;

fn abort(message: &str) {
    eprintln!("{}", message);
    std::process::exit(1);
}

fn wait_for<F>(f: F)
where
    F: Fn() -> bool,
{
    let mut done = false;
    while !done {
        done = f();
        sleep(Duration::from_secs_f32(0.05));
    }
}

fn wait_for_change(sway_command: &str) {
    let old = get_active_window();
    swaymsg(sway_command);
    eprintln!("waiting for {}...", sway_command);
    wait_for(|| {
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

fn swaymsg(sway_command: &str) {
    let StdoutTrimmed(result) = run_output!("swaymsg", sway_command);
    let expected = "[\n  {\n    \"success\": true\n  }\n]";
    if result != expected {
        abort(&format!("{:?} != {:?}", result, expected));
    }
}

#[derive(StructOpt)]
struct Args {}

fn main() {
    let vcs_type = VcsType::get();
    eprintln!("{:?}", vcs_type);
    let _args = Args::from_args();
    swaymsg("layout tabbed");
    wait_for_change("exec firefox --new-window github.com/notifications");
    swaymsg("move left");
    swaymsg("split toggle");
    swaymsg("layout tabbed");
    swaymsg("focus right");
    // right shell for git
    wait_for_change(&sway_terminal_command(None));
    swaymsg("split toggle");
    swaymsg("layout splith");
    wait_for_change(&sway_terminal_command(match vcs_type {
        VcsType::None => None,
        VcsType::Git => Some("git-watch-tree"),
        VcsType::Jujutsu => Some("/usr/bin/watch jj log --color always"),
    }));
    swaymsg("split toggle");
    swaymsg("layout splitv");
    wait_for_change(&sway_terminal_command(match vcs_type {
        VcsType::None => None,
        VcsType::Git => Some("/usr/bin/watch --color git -c color.ui=always status"),
        VcsType::Jujutsu => {
            Some("/usr/bin/watch bash -c '\"jj status --color always && jj diff --color always\"'")
        }
    }));
    swaymsg("focus left");
    swaymsg("focus left");
    // middle tab
    swaymsg("split toggle");
    swaymsg("layout splith");
    // shell top right
    wait_for_change(&sway_terminal_command(None));
    swaymsg("split toggle");
    swaymsg("layout splitv");
    swaymsg("focus left");
    swaymsg("split toggle");
    swaymsg("layout stacking");
    swaymsg("resize set 60 ppt 0");
    eprintln!("Done.");
}

const TERMINAL_COMMAND: &str = "alacritty";

fn sway_terminal_command(command: Option<&str>) -> String {
    let StdoutTrimmed(pwd) = run_output!("pwd");
    let mut result = format!("exec {} --working-directory {}", TERMINAL_COMMAND, pwd);
    if let Some(command) = command {
        result.push_str(&format!(" -e {}", command));
    }
    result
}

#[derive(Debug)]
enum VcsType {
    None,
    Git,
    Jujutsu,
}

impl VcsType {
    fn get() -> VcsType {
        if Path::new("./.jj").exists() {
            VcsType::Jujutsu
        } else if Path::new("./.git").exists() {
            VcsType::Git
        } else {
            VcsType::None
        }
    }
}
