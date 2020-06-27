use super::{LangExecutor, CompiledLangExecutor, ProjectInfo};
use crate::rmd::lang::{create_lang_dir, write_content_to_file, build_key_value_from_comment, parse_deps};
use std::process;
use std::process::Command;
use std::path::PathBuf;

pub struct RustExec {
    filename: String,
    origin: String,
    source_code: String,
    dir: String,
    dir_buf: PathBuf,
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
            dir_buf: Default::default(),
            output_dir: "".to_string(),
            project: ProjectInfo::new(),
        }
    }

    fn create_cargo_project(&self) -> String {
        let mut default_package = "[package]
name = \"hello_world\"
version = \"0.1.0\"

[dependencies]
".to_owned();

        for dep in self.project.deps.clone() {
            let result = format!("{} = \"{}\"\n", dep.name, dep.version);
            let pkg = default_package.push_str(&result);
        }

        write_content_to_file(default_package.clone(), self.dir_buf.join("Cargo.toml"));
        default_package
    }
}

impl LangExecutor for RustExec {
    fn parse_project_info(&mut self) -> ProjectInfo {
        let map = build_key_value_from_comment(self.source_code.clone());
        let mut project_info = ProjectInfo::new();
        project_info.name = String::from("hello");

        for (key, value) in map {
            match &key[..] {
                "deps" => {
                    project_info.deps = parse_deps(value.clone());
                }
                "name" => {
                    project_info.name = String::from(value.clone());
                }
                _ => {}
            }
        }

        project_info
    }
    fn build_project(&mut self) {
        let mut dir = create_lang_dir(String::from("rust"), String::from(self.project.name.clone()));
        let mut output = dir.clone();

        self.dir_buf = dir.clone();

        dir.push("main.rs");
        output.push("main");

        self.dir = write_content_to_file(self.source_code.clone(), dir);
        self.output_dir = output.into_os_string().into_string().unwrap();

        self.create_cargo_project();
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
// rinput-name: demo
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
    fn should_parse_project_info() {
        let mut exec = RustExec::new(String::from(get_hello_world_code()));
        exec.execute();

        assert_eq!("demo", exec.project.name)
    }

    #[test]
    fn should_success_run_hello_world() {
        let mut exec = RustExec::new(String::from("// rinput-name: hello2
fn main() {println!(\"Hello World!\");}
"));
        let mut cmd = exec.execute();
        let mut child = process::Command::new(exec.output_dir);
        let result = child.spawn().unwrap().wait().unwrap();

        assert_eq!(0, result.code().unwrap())
    }

    #[test]
    fn should_create_cargo_tomal() {
        let mut exec = RustExec::new(String::from(get_hello_world_code()));
        exec.execute();
        let dep = exec.create_cargo_project();

        assert_eq!("[package]
name = \"hello_world\"
version = \"0.1.0\"

[dependencies]
colored = \"1.8.0\"
", String::from(dep))
    }
}
