use std::{fs, process};
use std::path::PathBuf;
use std::process::Command;

use crate::rmd::lang::{
    CompiledLangExecutor,
    LangExecutor,
    ProjectInfo,
    build_key_value_from_comment,
    create_lang_dir,
    parse_deps,
    write_content_to_file,
};

pub struct JavaExec {
    lang: String,
    lang_prefix: String,
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
            lang: "java".to_string(),
            lang_prefix: "java".to_string(),
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
        let map = build_key_value_from_comment(self.source_code.clone());
        let mut project_info = ProjectInfo::new();

        self.filename = String::from("main");
        project_info.name = String::from("hello");

        for (key, value) in map {
            match &key[..] {
                "deps" => {
                    project_info.deps = parse_deps(value.clone());
                }
                "name" => {
                    project_info.name = String::from(value.clone());
                }
                "filename" => {
                    self.filename = String::from(value.clone());
                }
                _ => {}
            }
        }

        project_info
    }

    fn build_project(&mut self) {
        let mut base_dir = create_lang_dir(self.lang.clone(), self.project.name.clone());
        let mut output = base_dir.clone();

        let mut dir = base_dir.clone().join("src");
        fs::create_dir_all(dir.clone()).unwrap();

        self.dir_buf = base_dir.clone();

        dir.push(self.filename.clone() + &"." + &self.lang_prefix.clone());
        output.push(self.filename.clone());

        self.dir = write_content_to_file(self.source_code.clone(), dir);
    }

    fn install_dependency(&self) {}

    fn try_run(&self) {}

    fn execute(&mut self) -> Command {
        self.project = self.parse_project_info();
        self.build_project();
        let child = self.compile();
        child
    }
}

impl CompiledLangExecutor for JavaExec {
    fn compile(&self) -> Command {
        let mut child = process::Command::new("javac");
        child.arg(self.dir.clone());
        child.spawn().unwrap().wait().unwrap();

        let mut child = process::Command::new("java");
        let buf = self.dir_buf.join(self.filename.clone() + &".class");
        let string = buf.into_os_string().into_string().unwrap();
        println!("{}", string);
        child.arg(string);

        child
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn should_parse_project_deps() {}
}