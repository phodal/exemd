use std::path::PathBuf;
use std::{process, fs};
use std::process::Command;

use crate::rmd::lang::{LangExecutor, ProjectInfo, create_lang_dir, write_content_to_file, CompiledLangExecutor};

pub struct KotlinExec {
    lang: String,
    lang_prefix: String,
    source_code: String,
    dir: String,
    dir_buf: PathBuf,
    project: ProjectInfo,
}

impl KotlinExec {
    pub fn new(source: String) -> KotlinExec {
        KotlinExec {
            lang: "kotlin".to_string(),
            lang_prefix: "kt".to_string(),
            source_code: source.to_string(),
            dir: "".to_string(),
            dir_buf: Default::default(),
            project: ProjectInfo::from_code(source),
        }
    }
}

impl LangExecutor for KotlinExec {
    fn build_project(&mut self) {
        let base_dir = create_lang_dir(self.lang.clone(), self.project.name.clone());
        let mut output = base_dir.clone();

        let mut dir = base_dir.clone();
        fs::create_dir_all(dir.clone()).unwrap();

        self.dir_buf = base_dir.clone();

        dir.push(self.project.filename.clone() + &"." + &self.lang_prefix.clone());
        output.push(self.project.filename.clone());

        self.dir = write_content_to_file(self.source_code.clone(), dir.clone());
        println!("{}", dir.clone().into_os_string().into_string().unwrap())
    }

    fn install_dependency(&self) {}

    fn try_run(&self) {}

    fn execute(&mut self) -> Command {
        self.build_project();
        let child = self.compile();
        child
    }
}

impl CompiledLangExecutor for KotlinExec {
    fn compile(&self) -> Command {
        let mut child = process::Command::new("kotlinc");
        let string = format!("{}.{}", self.project.filename, "jar");
        let mut out_buf = self.dir_buf.clone();
        out_buf.push(string);

        let output = out_buf.into_os_string().into_string().unwrap();
        println!("{}", output.clone());

        child.arg(self.dir.clone())
            .arg("-include-runtime")
            .arg("-d")
            .arg(output.clone());

        child.spawn().unwrap().wait().unwrap();

        let mut result = process::Command::new("java");
        result.arg("-jar").arg(output.clone());

        result
    }
}

#[cfg(test)]
mod test {
    use crate::rmd::lang::{LangExecutor, KotlinExec};

    // todo: fix testing in ci
    #[test]
    fn should_success_run_kotlin_hello_world() {
        let mut exec = KotlinExec::new(get_hello_world());
        let mut cmd = exec.execute();
        assert_eq!(0, cmd.spawn().unwrap().wait().unwrap().code().unwrap())
    }

    fn get_hello_world() -> String {
        "fun main(args: Array<String>) {
    println(\"hello, world!\")
}
".to_owned()
    }

    // todo: fixed in ci
    #[test] #[ignore]
    fn should_success_run_kotlin() {
        let mut exec = KotlinExec::new(get_hello_world());
        let mut child = exec.execute();
        let out = child.output().expect("failed to execute process");

        child.spawn().unwrap().wait().unwrap();

        assert_eq!("hello, world!
", String::from_utf8_lossy(&out.stdout));
    }
}
