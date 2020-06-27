use super::{LangExecutor, CompiledLangExecutor, ProjectInfo};
use crate::rmd::lang::{create_lang_dir, write_content_to_file};
use std::process;

pub struct RustExec {
    filename: String,
    origin: String,
    source_code: String,
    dir: String,
    pub(crate) output_dir: String,
}

impl RustExec {
    pub fn new(mut source: String) -> RustExec {
        RustExec {
            filename: "".to_string(),
            origin: source.to_string(),
            source_code: source.to_string(),
            dir: "".to_string(),
            output_dir: "".to_string(),
        }
    }
}

impl LangExecutor for RustExec {
    fn parse_project_info(&self) -> ProjectInfo {
        ProjectInfo {
            deps: vec![],
            name: "".to_string(),
        }
    }
    fn build_project(&mut self) {
        let mut dir = create_lang_dir(String::from("rust"));
        let mut output = dir.clone();

        dir.push("hello.rs");
        output.push("hello");

        self.dir = write_content_to_file(self.source_code.clone(), dir);
        self.output_dir = output.into_os_string().into_string().unwrap();
    }
    fn install_dependency(&self) {}
    fn try_run(&self) {}
    fn execute(&mut self) {
        self.build_project();
        self.compile();
    }
}

impl CompiledLangExecutor for RustExec {
    fn compile(&self) {
        let mut child = process::Command::new("rustc");
        child.arg(self.dir.clone()).arg("-o").arg(self.output_dir.clone());
        child.spawn().unwrap().wait();

        println!("{}", self.output_dir.clone());
    }
}
