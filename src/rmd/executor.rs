use std::{env, fs, process};
use std::fmt::Debug;
use std::fs::{canonicalize, File, Metadata};
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom, Write};
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process::ExitStatus;

use tempfile::{NamedTempFile, TempDir, tempdir_in};

use crate::main;
use crate::rmd::command::Command;
use crate::rmd::lang::{LangExecutor, PythonExec, RustExec};

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
            let mut py_exec = PythonExec::new(source.clone());
            let child = py_exec.execute();
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
            let mut rustexec = RustExec::new(source.clone());
            let child = rustexec.execute();
            child
        }
        _ => {
            let mut child = process::Command::new(executor);
            child.arg("-c").arg(source);
            child
        }
    }
}
