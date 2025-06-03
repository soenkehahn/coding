use crate::VcsType;
use cradle::prelude::*;

pub const TERMINAL_COMMAND: &str = "alacritty";

pub fn terminal_command(command: Option<Vec<String>>) -> impl Input {
    let StdoutTrimmed(pwd) = run_output!("pwd");
    let mut result = vec![
        TERMINAL_COMMAND.to_owned(),
        "--working-directory".to_owned(),
        pwd,
    ];
    if let Some(command) = command {
        result.push("-e".to_owned());
        result.extend(command);
    }
    result
}

pub fn tree(vcs_type: VcsType) -> Option<Vec<String>> {
    match vcs_type {
        VcsType::None => None,
        VcsType::Git => Some(vec!["git-watch-tree".to_owned()]),
        VcsType::Jujutsu => Some(
            vec!["/usr/bin/watch", "jj", "log", "--color", "always"]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
        ),
    }
}

pub fn status(vcs_type: VcsType) -> Option<Vec<String>> {
    match vcs_type {
        VcsType::None => None,
        VcsType::Git => Some(
            "/usr/bin/watch --color git -c color.ui=always status"
                .split_whitespace()
                .map(ToString::to_string)
                .collect(),
        ),
        VcsType::Jujutsu => Some(
            "/usr/bin/watch bash -c '\"jj status --color always && jj diff --color always\"'"
                .split_whitespace()
                .map(ToString::to_string)
                .collect(),
        ),
    }
}
