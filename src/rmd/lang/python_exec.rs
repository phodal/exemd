use crate::rmd::lang::{LangExecutor, ProjectInfo};
use std::process;
use std::process::Command;

pub struct PythonExec {
    filename: String,
    origin: String,
    source_code: String,
    dir: String,
    pub(crate) output_dir: String,
}

impl PythonExec {
    pub fn new(mut source: String) -> PythonExec {
        PythonExec {
            filename: "".to_string(),
            origin: source.to_string(),
            source_code: source.to_string(),
            dir: "".to_string(),
            output_dir: "".to_string(),
        }
    }
}

impl LangExecutor for PythonExec {
    fn parse_project_info(&self) -> ProjectInfo {
        unimplemented!()
    }

    fn build_project(&mut self) {
        unimplemented!()
    }

    fn install_dependency(&self) {
        unimplemented!()
    }

    fn try_run(&self) {
        unimplemented!()
    }

    fn execute(&mut self) -> Command {
        let mut child = process::Command::new("python");
        child.arg("-c").arg(self.source_code.clone());

        child
    }
}