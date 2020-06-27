use super::{LangExecutor, ProjectInfo};

pub struct RustExec {

}

impl LangExecutor for RustExec {
    fn parse_project_info(&self) -> ProjectInfo {
        ProjectInfo {
            deps: vec![],
            name: "".to_string()
        }
    }
    fn build_project(&self) {

    }
    fn execute(&self) {

    }
}