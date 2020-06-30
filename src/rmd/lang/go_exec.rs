use std::{fs, process};
use std::path::PathBuf;
use std::process::Command;

use crate::rmd::lang::{CompiledLangExecutor, create_lang_dir, LangExecutor, ProjectInfo, write_content_to_file};

pub struct GoExec {
    lang: String,
    lang_prefix: String,
    source_code: String,
    dir: String,
    dir_buf: PathBuf,
    project: ProjectInfo,
}

impl GoExec {
    pub fn new(source: String) -> GoExec {
        GoExec {
            lang: "go".to_string(),
            lang_prefix: "go".to_string(),
            source_code: source.to_string(),
            dir: "".to_string(),
            dir_buf: Default::default(),
            project: ProjectInfo::from_code(source),
        }
    }
}

impl LangExecutor for GoExec {
    fn build_project(&mut self) {
        let base_dir = create_lang_dir(self.lang.clone(), self.project.name.clone());
        let mut output = base_dir.clone();

        let mut dir = base_dir.clone();
        fs::create_dir_all(dir.clone()).unwrap();

        self.dir_buf = base_dir;

        dir.push(self.project.filename.clone() + "." + &self.lang_prefix.clone());
        output.push(self.project.filename.clone());

        self.dir = write_content_to_file(self.source_code.clone(), dir.clone());
        println!("{}", dir.into_os_string().into_string().unwrap())
    }

    fn install_dependency(&self) {}

    fn try_run(&self) {}

    fn execute(&mut self) -> Command {
        self.build_project();
        self.compile()
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

#[cfg(test)]
mod test {
    use crate::rmd::lang::{LangExecutor, GoExec};

    #[test]
    fn should_success_run_go_hello_world() {
        let mut exec = GoExec::new(String::from(
            "package main

import \"fmt\"

func main() {
    fmt.Println(\"hello, world!\")
}
",
        ));

        let mut child = exec.execute();
        let out = child.output().expect("failed to execute process");
        let spawn = child.spawn().unwrap().wait();

        assert_eq!(0, spawn.unwrap().code().unwrap());
        assert_eq!("hello, world!
", String::from_utf8_lossy(&out.stdout));
    }
}
