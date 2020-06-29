use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

use regex::{Regex};

pub use self::java_exec::JavaExec;
pub use self::python_exec::PythonExec;
pub use self::rust_exec::RustExec;
pub use self::go_exec::GoExec;
pub use self::kotlin_exec::KotlinExec;

mod java_exec;
mod python_exec;
mod rust_exec;
mod go_exec;
mod kotlin_exec;

#[derive(Clone, Debug)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub artifact_id: String,
    pub group_id: String,
}

#[derive(Clone, Debug)]
pub struct ProjectInfo {
    pub deps: Vec<Dependency>,
    pub name: String,
    pub filename: String,
}

impl ProjectInfo {
    fn new() -> ProjectInfo {
        ProjectInfo {
            deps: vec![],
            name: "".to_string(),
            filename: "".to_string(),
        }
    }

    pub fn from_code(string: String) -> ProjectInfo {
        let map = build_key_value_from_comment(string.clone());
        let mut project_info = ProjectInfo::new();

        project_info.filename = String::from("main");
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
                    project_info.filename = String::from(value.clone());
                }
                _ => {}
            }
        }

        project_info
    }
}

pub trait LangExecutor {
    fn build_project(&mut self);
    fn install_dependency(&self);
    fn try_run(&self);
    fn execute(&mut self) -> Command;
}

pub trait CompiledLangExecutor: LangExecutor {
    fn compile(&self) -> Command;
}


pub fn write_content_to_file(source: String, dir: PathBuf) -> String {
    let mut f = File::create(dir.clone()).unwrap();
    f.write_all(source.as_ref()).unwrap();
    let code_path = dir.into_os_string().into_string().unwrap();

    code_path
}

pub fn create_lang_dir(lang: String, project_name: String) -> PathBuf {
    let dir = env::temp_dir()
        .join("com.phodal.exemd")
        .join(lang)
        .join(project_name);

    fs::create_dir_all(dir.clone()).unwrap();

    dir
}

pub fn build_key_value_from_comment(str: String) -> HashMap<String, String> {
    let mut info = HashMap::new();
    let re = Regex::new(r"(?x)//\s?exemd-(?P<key>([a-zA-z]+)):\s?(?P<value>(.*))").unwrap();
    let split = str.split("\n");
    let vec: Vec<&str> = split.collect();

    for line in vec {
        match re.captures(&line) {
            None => {}
            Some(caps) => {
                let key = &caps["key"];
                let value = &caps["value"];

                info.insert(String::from(key), String::from(value));
            }
        }
    }

    info
}

pub fn parse_deps(str: String) -> Vec<Dependency> {
    let split = str.split(",");
    let vec: Vec<&str> = split.collect();
    let re =
        Regex::new(r"(?x)(?P<name>([a-zA-Z-:]+))(;(?P<key>(\w+))=(?P<version>([a-zA-Z0-9.]+)))?")
            .unwrap();

    let mut deps: Vec<Dependency> = Vec::new();
    for line in vec {
        match re.captures(&line) {
            None => {}
            Some(caps) => {
                let name = &caps["name"];
                let version = &caps["version"];

                let dep = Dependency {
                    name: String::from(name),
                    version: String::from(version),
                    artifact_id: "".to_string(),
                    group_id: "".to_string(),
                };
                deps.push(dep);
            }
        }
    }

    deps
}

#[cfg(test)]
mod test {
    use crate::rmd::lang::{build_key_value_from_comment, parse_deps, ProjectInfo};

    #[test]
    fn should_parse_key_values() {
        let string = String::from("// exemd-deps: colored;version=1.8.0");
        let map = build_key_value_from_comment(string);

        assert_eq!(1, map.len());
        let value = map.get("deps").unwrap();
        assert_eq!(&"colored;version=1.8.0", value);
    }

    #[test]
    fn should_parse_one_dep() {
        let string = String::from("    colored;version=1.8.0");
        let deps = parse_deps(string);

        assert_eq!(1, deps.len());
        let first_dep = deps.get(0).unwrap();
        assert_eq!("colored", first_dep.name);
        assert_eq!("1.8.0", first_dep.version);
    }

    #[test]
    fn should_parse_deps() {
        let string = String::from("colored;version=1.8.0, pulldown-cmark;version=0.7");
        let deps = parse_deps(string);

        assert_eq!(2, deps.len());
        let first_dep = deps.get(1).unwrap();
        assert_eq!("pulldown-cmark", first_dep.name);
        assert_eq!("0.7", first_dep.version);
    }

    #[test]
    fn should_parse_java_deps() {
        let string = String::from("joda-time:joda-time;version=2.2");
        let deps = parse_deps(string);

        assert_eq!(1, deps.len());
        let first_dep = deps.get(0).unwrap();
        assert_eq!("joda-time:joda-time", first_dep.name);
        assert_eq!("2.2", first_dep.version);
    }


    fn get_hello_world_code() -> &'static str {
        "// exemd-deps: colored;version=1.8.0
// exemd-filename: main
// exemd-name: hello2
fn main() {
  println!(\"Hello World!\");
}
"
    }

    #[test]
    fn should_success_build_project_info() {
        let project = ProjectInfo::from_code(String::from(get_hello_world_code()));
        assert_eq!("main", project.filename.clone());
        assert_eq!("hello2", project.name.clone());
        assert_eq!(1, project.deps.len());
        assert_eq!("colored", project.deps[0].name);
        assert_eq!("1.8.0", project.deps[0].version);
    }
}
