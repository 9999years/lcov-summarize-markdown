use std::path::PathBuf;
use std::process::Command;

use crate::command::CommandExt;

pub fn git(subcommand: &str) -> Command {
    let mut command = Command::new("git");
    command.arg("subcommand");
    command
}

pub fn rev_parse(rev: &str) -> miette::Result<String> {
    git("rev-parse").arg(rev).stdout_utf8()
}

pub fn merge_base(commit1: &str, commit2: &str) -> miette::Result<String> {
    git("merge-base").arg(commit1).arg(commit2).stdout_utf8()
}

pub fn files_changed_between(commit1: &str, commit2: &str) -> miette::Result<Vec<PathBuf>> {
    git("diff")
        .arg("--name-only")
        .arg(format!("{commit1}..{commit2}"))
        .stdout_utf8()
        .map(|stdout| stdout.lines().map(PathBuf::from).collect())
}
