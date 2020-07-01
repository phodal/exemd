use std::{process};
use std::io::{Error, ErrorKind};
use std::io::Result;
use std::process::{ExitStatus};

use crate::rmd::command::Command;
use crate::rmd::lang::{JavaExec, LangExecutor, PythonExec, RustExec, GoExec, KotlinExec};

pub fn execute_command(cmd: Command) -> Result<ExitStatus> {
    if cmd.script.source == "" {
        let msg = "Command has no script.";
        return Err(Error::new(ErrorKind::Other, msg));
    }

    if cmd.script.executor == "" {
        let msg = "Command script requires a lang code which determines which executor to use.";
        return Err(Error::new(ErrorKind::Other, msg));
    }

    let mut child = prepare_command(&cmd);
    // child.stdout(std::process::Stdio::piped()) // set up stdout so we can read it
    //     .spawn()?
    //     .wait()
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
        "ts" | "typescript" => {
            let mut child;
            child = process::Command::new("deno");
            child.arg("eval").arg(source);
            child
        }
        "py" | "python" => {
            let mut py_exec = PythonExec::new(source);
            py_exec.execute()
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
            let mut rustexec = RustExec::new(source);
            rustexec.execute()
        }
        "java" => {
            let mut javaexec = JavaExec::new(source);
            javaexec.execute()
        }
        "go" => {
            let mut exec = GoExec::new(source);
            exec.execute()
        }
        "kotlin" | "k" => {
            let mut exec = KotlinExec::new(source);
            exec.execute()
        }
        "cli" => {
            let split = source.split(' ');
            let vec: Vec<&str> = split.collect();
            let first = vec[0].clone();
            let mut copy = vec.clone();
            copy.remove(0);

            let mut args: Vec<String> = Vec::new();
            for arg in copy {
                args.push(arg.replace("\n", ""));
            }

            let mut child = process::Command::new(first);
            child.args(args);

            child
        }
        _ => {
            let mut child = process::Command::new(executor);
            child.arg("-c").arg(source);
            child
        }
    }
}
