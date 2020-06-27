use super::{LangExecutor, CompiledLangExecutor, ProjectInfo};
use crate::rmd::lang::{create_lang_dir, write_content_to_file, build_key_value_from_comment, parse_deps};
use std::process;
use std::process::Command;

pub struct RustExec {
    filename: String,
    origin: String,
    source_code: String,
    dir: String,
    pub(crate) output_dir: String,
    project: ProjectInfo,
}

impl RustExec {
    pub fn new(mut source: String) -> RustExec {
        RustExec {
            filename: "".to_string(),
            origin: source.to_string(),
            source_code: source.to_string(),
            dir: "".to_string(),
            output_dir: "".to_string(),
            project: ProjectInfo::new(),
        }
    }
}

impl LangExecutor for RustExec {
    fn parse_project_info(&mut self) -> ProjectInfo {
        let map = build_key_value_from_comment(self.source_code.clone());
        let mut project_info = ProjectInfo::new();

        for (key, value) in map {
            if key == String::from("deps") {
                project_info.deps = parse_deps(value);
            }
        }

        project_info
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
    fn execute(&mut self) -> Command {
        self.project = self.parse_project_info();
        self.build_project();
        let child = self.compile();
        child
    }
}

impl CompiledLangExecutor for RustExec {
    fn compile(&self) -> Command {
        let mut child = process::Command::new("rustc");
        child.arg(self.dir.clone()).arg("-o").arg(self.output_dir.clone());
        child.spawn().unwrap().wait();

        println!("{}", self.output_dir.clone());
        child
    }
}

#[cfg(test)]
mod test {
    use crate::rmd::lang::{RustExec, LangExecutor};
    use std::process;


    fn get_hello_world_code() -> &'static str {
        "// rinput-deps: colored;version=1.8.0
fn main() {
  println!(\"Hello World!\");
}
"
    }

    #[test]
    fn should_parse_project_deps() {
        let mut exec = RustExec::new(String::from(get_hello_world_code()));
        exec.execute();

        assert_eq!(1, exec.project.deps.len())
    }

    #[test]
    fn should_success_run_hello_world() {
        let mut exec = RustExec::new(String::from(get_hello_world_code()));
        let mut cmd = exec.execute();
        let mut child = process::Command::new(exec.output_dir);
        let result = child.spawn().unwrap().wait().unwrap();

        assert_eq!(0, result.code().unwrap())
    }
}
