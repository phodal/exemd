use std::path::PathBuf;
use crate::rmd::lang::{ProjectInfo, LangExecutor, CompiledLangExecutor};
use std::process::Command;

pub struct JavaExec {
    filename: String,
    origin: String,
    source_code: String,
    dir: String,
    dir_buf: PathBuf,
    project: ProjectInfo,
}

impl JavaExec {
    pub fn new(mut source: String) -> JavaExec {
        JavaExec {
            filename: "".to_string(),
            origin: source.to_string(),
            source_code: source.to_string(),
            dir: "".to_string(),
            dir_buf: Default::default(),
            project: ProjectInfo::new(),
        }
    }
}

impl LangExecutor for JavaExec {
    fn parse_project_info(&mut self) -> ProjectInfo {
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
        unimplemented!()
    }
}

impl CompiledLangExecutor for JavaExec {
    fn compile(&self) -> Command {
        unimplemented!()
    }
}
