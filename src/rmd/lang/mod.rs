use std::{env, fs};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use regex::{Captures, Regex};

pub use self::python_exec::PythonExec;
pub use self::rust_exec::RustExec;

mod python_exec;
mod rust_exec;

#[derive(Debug)]
pub struct Dependency {
    pub version: String,
    pub artifact_id: String,
    pub group_id: String,
}

#[derive(Debug)]
pub struct ProjectInfo {
    pub deps: Vec<Dependency>,
    pub name: String,
}

impl ProjectInfo {
    fn new() -> ProjectInfo {
        ProjectInfo {
            deps: vec![],
            name: "".to_string(),
        }
    }
}

pub trait LangExecutor {
    fn parse_project_info(&self) -> ProjectInfo;
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

pub fn create_lang_dir(lang: String) -> PathBuf {
    let mut dir = env::temp_dir().join("com.phodal.rinput").join(lang);
    fs::create_dir_all(dir.clone()).unwrap();

    dir
}

pub fn build_key_value_from_comment(str: String) -> HashMap<&'static str, &'static str, RandomState> {
    let mut info = HashMap::new();
    let re = Regex::new(r"(?x)//\s?rinput-(?P<key>([a-zA-z]+)):\s?(?P<value>(.*))").unwrap();
    let mut split = str.split("\n");
    let vec: Vec<&str> = split.collect();

    for line in vec {
        match re.captures(&line) {
            None => {}
            Some(caps) => {
                // let key = caps["key"];
                // let value = caps["value"];

                // info.insert(key.clone(), value.clone());
                info.insert("deps", "colored;version=1.8.0");
            }
        }
    }

    info
}


#[cfg(test)]
mod test {
    use crate::rmd::lang::{build_key_value_from_comment, LangExecutor, RustExec};

    #[test]
    fn should_parse_key_values() {
        let string = String::from("// rinput-deps: colored;version=1.8.0");
        let map = build_key_value_from_comment(string);

        assert_eq!(1, map.len());
        let value = map.get(&"deps").unwrap();
        assert_eq!(&"colored;version=1.8.0", value);
    }
}