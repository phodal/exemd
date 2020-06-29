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
    fn build_project(&mut self) {
    }

    fn install_dependency(&self) {
    }

    fn try_run(&self) {
    }

    fn execute(&mut self) -> Command {
        let mut child = process::Command::new("python");
        child.arg("-c").arg(self.source_code.clone());

        child
    }
}
