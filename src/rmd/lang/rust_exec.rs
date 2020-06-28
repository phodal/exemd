use super::{LangExecutor, CompiledLangExecutor, ProjectInfo};
use crate::rmd::lang::{create_lang_dir, write_content_to_file, build_key_value_from_comment, parse_deps};
use std::{process, fs};
use std::process::Command;
use std::path::PathBuf;

pub struct RustExec {
    lang: String,
    lang_prefix: String,
    filename: String,
    origin: String,
    source_code: String,
    dir: String,
    dir_buf: PathBuf,
    project: ProjectInfo,
}

impl RustExec {
    pub fn new(mut source: String) -> RustExec {
        RustExec {
            lang: "rust".to_string(),
            lang_prefix: "rs".to_string(),
            filename: "".to_string(),
            origin: source.to_string(),
            source_code: source.to_string(),
            dir: "".to_string(),
            dir_buf: Default::default(),
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
            default_package.push_str(&result);
        }

        write_content_to_file(default_package.clone(), self.dir_buf.join("Cargo.toml"));
        default_package
    }
}

impl LangExecutor for RustExec {
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

        dir.push(self.filename.clone() + &self.lang_prefix.clone());
        output.push(self.filename.clone());

        self.dir = write_content_to_file(self.source_code.clone(), dir);
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
        let path = self.dir_buf.join("Cargo.toml").into_os_string().into_string().unwrap();
        let mut child = process::Command::new("cargo");
        child.arg("run").arg("--manifest-path").arg(path.clone());

        println!("{}", path.clone());
        child
    }
}

#[cfg(test)]
mod test {
    use crate::rmd::lang::{RustExec, LangExecutor};
    use std::process;

    fn get_hello_world_code() -> &'static str {
        "// exemd-deps: colored;version=1.8.0
// exemd-name: demo
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
        let mut exec = RustExec::new(String::from("// exemd-name: hello2
fn main() {println!(\"Hello World!\");}
"));
        let mut cmd = exec.execute();
        assert_eq!(0, cmd.spawn().unwrap().wait().unwrap().code().unwrap())
    }

    #[test]
    fn should_support_for_filename() {
        let mut exec = RustExec::new(String::from("// exemd-name: hello2
// exemd-filename: hello2
fn main() {println!(\"Hello World!\");}
"));
        exec.parse_project_info();
        assert_eq!("hello2", exec.filename.clone())
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
