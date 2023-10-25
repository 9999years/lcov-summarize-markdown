use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use clap::Parser;
use miette::miette;
use miette::Context;
use miette::IntoDiagnostic;

use crate::git;

#[derive(Parser, Debug)]
pub struct Opts {
    /// Files to highlight coverage information for, as in a GitHub PR.
    ///
    /// If `--changed-files` is given but no value is supplied, changed files are read from
    /// `git diff --name-only $HEAD..$(git merge-base $HEAD $MAIN)`, where `$HEAD` is the
    /// value of `--git-head` ("HEAD" by default) and `$MAIN` is the value of `--git-main`
    /// ("main" by default, but I'll try "master" and "trunk" too just to be nice).
    #[arg(long, num_args = 0..)]
    changed_files: Vec<PathBuf>,

    /// The Git `HEAD` revision to compare "from" when `--changed-files` is used without a
    /// value.
    #[arg(long, default_value = "HEAD", env = "GITHUB_HEAD_REF")]
    git_head: String,

    /// The Git `main` revision to compare "to" when `--changed-files` is used without a
    /// value.
    ///
    /// [default: I'll try `main`, `master`, and `trunk`]
    #[arg(long, env = "GITHUB_BASE_REF")]
    git_base: Option<String>,

    /// A path to write output to.
    ///
    /// [default: stdout]
    #[arg(long)]
    output: Option<PathBuf>,

    /// Coverage file to read.
    ///
    /// The coverage file should be in `lcov` format, although only line coverage is supported
    /// at the moment.
    #[arg(short, long)]
    pub coverage_file: PathBuf,
}

impl Opts {
    pub fn open_output(&self) -> miette::Result<Box<dyn Write>> {
        match &self.output {
            Some(path) => File::open(path)
                .into_diagnostic()
                .wrap_err_with(|| format!("Failed to open {path:?}"))
                .map(|file| Box::new(file) as Box<dyn Write>),
            None => Ok(Box::new(std::io::stdout())),
        }
    }

    pub fn changed_files(&self) -> miette::Result<Vec<PathBuf>> {
        if !self.changed_files.is_empty() {
            Ok(self.changed_files.clone())
        } else {
            self.git_changed_files()
        }
    }

    fn git_changed_files(&self) -> miette::Result<Vec<PathBuf>> {
        let merge_base = git::merge_base(&self.git_head, &self.git_base()?)?;

        git::files_changed_between(&self.git_head, &merge_base)
    }

    fn git_base(&self) -> miette::Result<String> {
        if let Some(base) = &self.git_base {
            return Ok(base.clone());
        }

        for branch in ["main", "master", "trunk"] {
            match git::rev_parse(branch) {
                Ok(_stdout) => {
                    return Ok(branch.to_owned());
                }
                Err(err) => {}
            }
        }

        Err(miette!(
            "Failed to find a main branch; maybe specify one with `--git-base`?"
        ))
    }
}
