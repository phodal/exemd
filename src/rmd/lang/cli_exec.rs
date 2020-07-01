use std::path::PathBuf;
use std::process::Command;
use std::{process};

use crate::rmd::lang::{
    LangExecutor, ProjectInfo,
};

#[allow(dead_code)]
pub struct CliExec {
    lang: String,
    lang_prefix: String,
    source_code: String,
    dir: String,
    dir_buf: PathBuf,
    project: ProjectInfo,
}

impl CliExec {
    pub fn new(source: String) -> CliExec {
        CliExec {
            lang: "".to_string(),
            lang_prefix: "".to_string(),
            source_code: source.to_string(),
            dir: "".to_string(),
            dir_buf: Default::default(),
            project: ProjectInfo::from_code(source),
        }
    }
}

impl LangExecutor for CliExec {
    fn build_project(&mut self) {}

    fn install_dependency(&self) {}

    fn try_run(&self) {}

    fn execute(&mut self) -> Command {
        let split = self.source_code.split(' ');
        let vec: Vec<&str> = split.collect();
        let first = vec[0];
        let mut copy = vec.clone();
        copy.remove(0);

        let mut args: Vec<String> = Vec::new();
        for arg in copy {
            args.push(arg.replace("\n", ""));
        }

        let mut child = process::Command::new(String::from(first));
        child.args(args);

        child
    }
}

#[cfg(test)]
mod test {}
