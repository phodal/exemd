mod rustexec;

#[derive(Debug)]
pub struct Dependency {

}

#[derive(Debug)]
pub struct ProjectInfo {
    pub deps: Vec<Dependency>,
    pub name: String,
}

pub trait LangExecutor {
    fn parse_project_info(&self) -> ProjectInfo;
    fn build_project(&self);
    fn install_dependency(&self);
    fn try_run(&self);
    fn execute(&self);
}
