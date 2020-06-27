use super::{LangExecutor, CompiledLangExecutor, ProjectInfo};

pub struct RustExec {}

impl RustExec {
    fn new() -> RustExec {
        RustExec {}
    }
}

impl LangExecutor for RustExec {
    fn parse_project_info(&self) -> ProjectInfo {
        ProjectInfo {
            deps: vec![],
            name: "".to_string(),
        }
    }
    fn build_project(&self) {}
    fn install_dependency(&self) {}
    fn try_run(&self) {}
    fn execute(&self) {}
}

impl CompiledLangExecutor for RustExec {
    fn compile(&self) {
        unimplemented!()
    }
}
