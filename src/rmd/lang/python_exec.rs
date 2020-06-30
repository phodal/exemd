use std::process;
use std::process::Command;

use crate::rmd::lang::{LangExecutor, ProjectInfo};
use std::path::PathBuf;

#[allow(dead_code)]
pub struct PythonExec {
    lang: String,
    lang_prefix: String,
    source_code: String,
    dir: String,
    dir_buf: PathBuf,
    project: ProjectInfo,
}

impl PythonExec {
    pub fn new(source: String) -> PythonExec {
        PythonExec {
            lang: "python".to_string(),
            lang_prefix: "py".to_string(),
            source_code: source.to_string(),
            dir: "".to_string(),
            dir_buf: Default::default(),
            project: ProjectInfo::from_code(source),
        }
    }
}

impl LangExecutor for PythonExec {
    fn build_project(&mut self) {}

    fn install_dependency(&self) {
        for dep in &self.project.deps.clone() {
            let dep_str = format!("{}=={}", dep.name, dep.version);
            let mut child = process::Command::new("pip");
            child.arg("install")
                .arg(dep_str);

            child.output().expect("failed to execute process");

            child.spawn().unwrap().wait().unwrap();
        }
    }

    fn try_run(&self) {
    }

    fn execute(&mut self) -> Command {
        let mut child = process::Command::new("python");
        self.install_dependency();
        child.arg("-c").arg(self.source_code.clone());

        child
    }
}

#[cfg(test)]
mod test {
    use crate::rmd::lang::{PythonExec, LangExecutor};

    #[test]
    fn should_success_run_python_hello_world() {
        let mut exec = PythonExec::new(String::from("print(\"hello, world!\")"));
        let mut child = exec.execute();
        let out = child.output().expect("failed to execute process");

        child.spawn().unwrap().wait().unwrap();

        assert_eq!("hello, world!
", String::from_utf8_lossy(&out.stdout));
    }

    #[test]
    fn should_success_run_python_with_dep_color() {
        let mut exec = PythonExec::new(String::from("# rmd-deps: termcolor
import sys
from termcolor import colored, cprint

text = colored('Hello, World!', 'red', attrs=['reverse', 'blink'])
print(text)
"));
        let mut child = exec.execute();
        let out = child.output().expect("failed to execute process");

        child.spawn().unwrap().wait().unwrap();

        assert_eq!("\u{1b}[5m\u{1b}[7m\u{1b}[31mHello, World!\u{1b}[0m
", String::from_utf8_lossy(&out.stdout));
    }
}