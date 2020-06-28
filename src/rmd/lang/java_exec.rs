use std::path::PathBuf;
use std::process::Command;
use std::{fs, process};

use crate::rmd::lang::{build_key_value_from_comment, create_lang_dir, parse_deps, write_content_to_file, CompiledLangExecutor, LangExecutor, ProjectInfo};

pub struct JavaExec {
    lang: String,
    lang_prefix: String,
    source_code: String,
    dir: String,
    dir_buf: PathBuf,
    project: ProjectInfo,
}

impl JavaExec {
    pub fn new(source: String) -> JavaExec {
        JavaExec {
            lang: "java".to_string(),
            lang_prefix: "java".to_string(),
            source_code: source.to_string(),
            dir: "".to_string(),
            dir_buf: Default::default(),
            project: ProjectInfo::from_code(source),
        }
    }

    fn create_dependency_file(&self) -> String {
        let mut default_package = "apply plugin: 'java'
apply plugin: 'application'

repositories {
    mavenCentral()
}

dependencies {
"
        .to_owned();

        for dep in self.project.deps.clone() {
            let result = format!("compile \"{}:{}\"", dep.name, dep.version);
            default_package.push_str(&result);
        }

        default_package.push_str("\n}\n\n");
        if self.project.name != String::from("") {
            default_package.push_str(&format!(
                "mainClassName = '{}.{}'\n",
                self.project.name.clone(),
                self.project.filename.clone()
            ));
        } else {
            default_package.push_str(&format!("mainClassName = 'main'"));
        }

        write_content_to_file(default_package.clone(), self.dir_buf.join("build.gradle"));
        default_package
    }
}

impl LangExecutor for JavaExec {
    fn build_project(&mut self) {
        let mut base_dir = create_lang_dir(self.lang.clone(), self.project.name.clone());
        let mut output = base_dir.clone();

        let mut dir = base_dir.clone().join("src").join("main").join("java");

        if self.project.name != String::from("") {
            dir.push(self.project.name.clone());
        }

        fs::create_dir_all(dir.clone()).unwrap();

        self.dir_buf = base_dir.clone();

        dir.push(self.project.filename.clone() + &"." + &self.lang_prefix.clone());
        output.push(self.project.filename.clone());

        self.dir = write_content_to_file(self.source_code.clone(), dir);
        self.create_dependency_file();
    }

    fn install_dependency(&self) {}

    fn try_run(&self) {}

    fn execute(&mut self) -> Command {
        self.build_project();
        let child = self.compile();
        child
    }
}

impl CompiledLangExecutor for JavaExec {
    fn compile(&self) -> Command {
        // support: gradle -p [path] run
        let mut child = process::Command::new("gradle");
        child.arg("-p").arg(self.dir_buf.clone()).arg("run");
        child
    }
}

#[cfg(test)]
mod test {
    use crate::rmd::lang::{JavaExec, LangExecutor};

    fn get_hello_world_code() -> &'static str {
        "java
// exemd-deps: joda-time:joda-time;version=2.2
package joda;

import org.joda.time.LocalTime;

public class HelloWorld {
  public static void main(String[] args) {
    LocalTime currentTime = new LocalTime();
    System.out.println(\"The current local time is: \" + currentTime);

    Greeter greeter = new Greeter();
    System.out.println(greeter.sayHello());
  }
}
"
    }

    #[test]
    fn should_build_normal_java_dep() {
        let mut exec = JavaExec::new(String::from(get_hello_world_code()));
        exec.execute();
        let dep = exec.create_dependency_file();

        assert_eq!(
            "apply plugin: 'java'
apply plugin: 'application'

repositories {
    mavenCentral()
}

dependencies {
compile \"joda-time:joda-time:2.2\"
}

mainClassName = 'hello.main'
",
            String::from(dep)
        )
    }

    #[test]
    fn should_build_naming_file_java_deps() {
        let mut exec = JavaExec::new(String::from(
            "// exemd-name: joda
// exemd-filename: HelloWorld
// exemd-deps: joda-time:joda-time;version=2.2
package joda;

import org.joda.time.LocalTime;

public class HelloWorld {
}
",
        ));
        exec.execute();
        let dep = exec.create_dependency_file();

        assert_eq!(
            "apply plugin: 'java'
apply plugin: 'application'

repositories {
    mavenCentral()
}

dependencies {
compile \"joda-time:joda-time:2.2\"
}

mainClassName = 'joda.HelloWorld'
",
            String::from(dep)
        )
    }
}
