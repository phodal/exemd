use std::process;
use std::process::Command;

use crate::rmd::lang::{LangExecutor, ProjectInfo};

pub struct PythonExec {
    filename: String,
    origin: String,
    source_code: String,
    dir: String,
}

impl PythonExec {
    pub fn new(mut source: String) -> PythonExec {
        PythonExec {
            filename: "".to_string(),
            origin: source.to_string(),
            source_code: source.to_string(),
            dir: "".to_string(),
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
