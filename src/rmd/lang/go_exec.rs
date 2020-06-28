use std::path::PathBuf;
use std::{process, fs};
use std::process::Command;

use crate::rmd::lang::{LangExecutor, ProjectInfo, create_lang_dir, write_content_to_file, CompiledLangExecutor};

pub struct GoExec {
    lang: String,
    lang_prefix: String,
    filename: String,
    origin: String,
    source_code: String,
    dir: String,
    dir_buf: PathBuf,
    project: ProjectInfo,
}

impl GoExec {
    pub fn new(mut source: String) -> GoExec {
        GoExec {
            lang: "go".to_string(),
            lang_prefix: "go".to_string(),
            filename: "".to_string(),
            origin: source.to_string(),
            source_code: source.to_string(),
            dir: "".to_string(),
            dir_buf: Default::default(),
            project: ProjectInfo::new(),
        }
    }
}

impl LangExecutor for GoExec {
    fn parse_project_info(&mut self, string: String) -> ProjectInfo {
        self.filename = String::from("main");

        let mut project_info = ProjectInfo::new();
        project_info.name = String::from("hello");
        project_info
    }

    fn build_project(&mut self) {
        let mut base_dir = create_lang_dir(self.lang.clone(), self.project.name.clone());
        let mut output = base_dir.clone();

        let mut dir = base_dir.clone();
        fs::create_dir_all(dir.clone()).unwrap();

        self.dir_buf = base_dir.clone();

        dir.push(self.filename.clone() + &"." + &self.lang_prefix.clone());
        output.push(self.filename.clone());

        self.dir = write_content_to_file(self.source_code.clone(), dir.clone());
        println!("{}", dir.clone().into_os_string().into_string().unwrap())
    }

    fn install_dependency(&self) {

    }

    fn try_run(&self) {

    }

    fn execute(&mut self) -> Command {
        self.project = self.parse_project_info(self.source_code.clone());
        self.build_project();
        let child = self.compile();
        child
    }
}

impl CompiledLangExecutor for GoExec {
    fn compile(&self) -> Command {
        let mut child = process::Command::new("go");
        child.arg("run").arg(self.dir.clone());
        println!("{}", self.dir.clone());

        child
    }
}
