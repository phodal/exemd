use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub use self::rust_exec::RustExec;
pub use self::python_exec::PythonExec;

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