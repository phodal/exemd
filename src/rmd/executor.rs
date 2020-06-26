use std::fs::canonicalize;
use std::io::Result;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process;
use std::process::ExitStatus;

use clap::crate_name;

use crate::rmd::command::Command;

pub fn execute_command(cmd: Command) -> Result<ExitStatus> {
    if cmd.script.source == String::from("") {
        let msg = "Command has no script.";
        return Err(Error::new(ErrorKind::Other, msg));
    }

    if cmd.script.executor == String::from("") {
        let msg = "Command script requires a lang code which determines which executor to use.";
        return Err(Error::new(ErrorKind::Other, msg));
    }

    let mut child = prepare_command(&cmd);
    child.spawn()?.wait()
}

fn prepare_command(cmd: &Command) -> process::Command {
    let executor = cmd.script.executor.clone();
    let source = cmd.script.source.clone();

    match executor.as_ref() {
        "js" | "javascript" => {
            let mut child;
            child = process::Command::new("node");
            child.arg("-e").arg(source);
            child
        }
        "py" | "python" => {
            let mut child = process::Command::new("python");
            child.arg("-c").arg(source);
            child
        }
        "rb" | "ruby" => {
            let mut child = process::Command::new("ruby");
            child.arg("-e").arg(source);
            child
        }
        "php" => {
            let mut child = process::Command::new("php");
            child.arg("-r").arg(source);
            child
        }
        "rust" => {
            // todo: support execute file
            let mut child = process::Command::new("rustc");
            child.arg("-l").arg(source);
            child
        }
        #[cfg(windows)]
        "cmd" | "batch" => {
            let mut child = process::Command::new("cmd.exe");
            child.arg("/c").arg(source);
            child
        }
        #[cfg(windows)]
        "powershell" => {
            let mut child = process::Command::new("powershell.exe");
            child.arg("-c").arg(source);
            child
        }
        // Any other executor that supports -c (sh, bash, zsh, fish, dash, etc...)
        _ => {
            let mut child = process::Command::new(executor);
            child.arg("-c").arg(source);
            child
        }
    }
}
