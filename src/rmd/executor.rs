use std::fs::{canonicalize, Metadata, File};
use std::io::{Error, ErrorKind, Write, SeekFrom, Seek, Read};
use std::io::Result;
use std::path::{Path, PathBuf};
use std::{process, fs, env};
use std::process::ExitStatus;

use tempfile::{NamedTempFile, TempDir, tempdir_in};

use crate::rmd::command::Command;
use crate::main;
use std::fmt::Debug;
use std::ffi::OsStr;

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
            let mut dir = create_lang_dir(String::from("rust"));
            let mut output = dir.clone();

            dir.push("hello.rs");
            let code_path = write_content_to_file(source, dir);
            output.push("hello");
            let mut output_path = output.into_os_string().into_string().unwrap();

            let mut child = process::Command::new("rustc");
            child.arg(code_path).arg("-o").arg(output_path.clone());
            child.spawn().unwrap().wait();

            println!("{}", output_path);
            child = process::Command::new(output_path.clone());

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

fn write_content_to_file(source: String, dir: PathBuf) -> String {
    let mut f = File::create(dir.clone()).unwrap();
    f.write_all(source.as_ref()).unwrap();
    let code_path = dir.into_os_string().into_string().unwrap();

    code_path
}

fn create_lang_dir(lang: String) -> PathBuf {
    let mut dir = env::temp_dir().join("com.phodal.rinput").join(lang);
    fs::create_dir_all(dir.clone()).unwrap();

    dir
}