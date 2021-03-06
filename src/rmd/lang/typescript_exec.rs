use std::path::PathBuf;
use std::process::Command;
use std::{process};

use crate::rmd::lang::{
    LangExecutor, ProjectInfo,
};

#[allow(dead_code)]
pub struct TypeScriptExec {
    lang: String,
    lang_prefix: String,
    source_code: String,
    dir: String,
    dir_buf: PathBuf,
    project: ProjectInfo,
}

impl TypeScriptExec {
    pub fn new(source: String) -> TypeScriptExec {
        TypeScriptExec {
            lang: "".to_string(),
            lang_prefix: "".to_string(),
            source_code: source.to_string(),
            dir: "".to_string(),
            dir_buf: Default::default(),
            project: ProjectInfo::from_code(source),
        }
    }
}

impl LangExecutor for TypeScriptExec {
    fn build_project(&mut self) {}

    fn install_dependency(&self) {}

    fn try_run(&self) {}

    fn execute(&mut self) -> Command {
        let mut child = process::Command::new("deno");
        child.arg("eval").arg(self.source_code.clone());
        child
    }
}

#[cfg(test)]
mod test {}
