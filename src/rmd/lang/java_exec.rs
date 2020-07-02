use std::path::PathBuf;
use std::process::Command;
use std::{fs, process};

use crate::rmd::lang::{
    create_lang_dir, write_content_to_file, CompiledLangExecutor, LangExecutor, ProjectInfo,
};

pub struct JavaExec {
    lang: String,
    lang_prefix: String,
    source_code: String,
    dir: String,
    dir_buf: PathBuf,
    project: ProjectInfo,
}

pub fn create_dependency_file(info: ProjectInfo, dir: PathBuf) -> String {
    let mut default_package = "apply plugin: 'java'
apply plugin: 'application'

repositories {
    mavenCentral()
}

dependencies {
"
        .to_owned();

    for dep in info.deps.clone() {
        let result = format!("compile \"{}:{}\"", dep.name, dep.version);
        default_package.push_str(&result);
    }

    default_package.push_str("\n}\n\n");
    if info.name != "" {
        default_package.push_str(&format!(
            "mainClassName = '{}.{}'\n",
            info.name.clone(),
            info.filename.clone()
        ));
    } else {
        default_package.push_str("mainClassName = 'main'");
    }

    write_content_to_file(default_package.clone(), dir.join("build.gradle"));
    default_package
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
}

impl LangExecutor for JavaExec {
    fn build_project(&mut self) {
        let base_dir = create_lang_dir(self.lang.clone(), self.project.name.clone());
        let mut output = base_dir.clone();

        let mut dir = base_dir.join("src").join("main").join("java");

        if self.project.name != "" {
            dir.push(self.project.name.clone());
        }

        fs::create_dir_all(dir.clone()).unwrap();

        self.dir_buf = base_dir;

        dir.push(self.project.filename.clone() + "." + &self.lang_prefix.clone());
        output.push(self.project.filename.clone());

        self.dir = write_content_to_file(self.source_code.clone(), dir);
        create_dependency_file(self.project.clone(), self.dir_buf.clone());
    }

    fn install_dependency(&self) {}

    fn try_run(&self) {}

    fn execute(&mut self) -> Command {
        self.build_project();
        self.compile()
    }
}

impl CompiledLangExecutor for JavaExec {
    fn compile(&self) -> Command {
        // support: gradle -p [path] run
        let output_path = self.dir_buf.clone().into_os_string().into_string().unwrap();
        let mut child = process::Command::new("gradle");
        child.arg("-p").arg(output_path.clone()).arg("run");

        println!("gradle -p {} run", output_path);
        child
    }
}

#[cfg(test)]
mod test {
    use crate::rmd::lang::{JavaExec, LangExecutor};
    use crate::rmd::lang::java_exec::create_dependency_file;

    fn get_joda_code() -> &'static str {
        "// exemd-deps: joda-time:joda-time;version=2.2
// exemd-name: joda
// exemd-filename: HelloWorld
package joda;

import org.joda.time.LocalTime;

public class HelloWorld {
  public static void main(String[] args) {
    LocalTime currentTime = new LocalTime();
    System.out.println(\"The current local time is: \" + currentTime);
  }
}
"
    }

    #[test]
    fn should_build_normal_java_dep() {
        let mut exec = JavaExec::new(String::from(get_joda_code()));
        exec.execute();
        let dep = create_dependency_file(exec.project.clone(), exec.dir_buf);

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
            dep
        )
    }

    #[test]
    fn should_build_naming_file_java_deps() {
        let mut exec = JavaExec::new(String::from(get_joda_code()));
        exec.execute();
        let dep = create_dependency_file(exec.project, exec.dir_buf);

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
            dep
        )
    }

    #[cfg(feature = "local")]
    mod local {
        use crate::rmd::lang::java_exec::test::get_joda_code;
        use crate::rmd::lang::{JavaExec, LangExecutor};

        #[test]
        fn should_success_run_java_hello_world() {
            let mut exec = JavaExec::new(String::from(
                "// exemd-name: hello
package hello;

public class main {
    public static void main(String[] args) {
        System.out.println(\"hello, world!\");
    }
}
",
            ));
            let mut child = exec.execute();
            let out = child.output().expect("failed to execute process");

            child.spawn().unwrap().wait().unwrap();

            assert_eq!(
                true,
                String::from_utf8_lossy(&out.stdout).contains("hello, world!")
            );
        }

        #[test]
        fn should_success_run_java_with_deps() {
            let mut exec = JavaExec::new(String::from(get_joda_code()));
            let mut child = exec.execute();
            let out = child.output().expect("failed to execute process");

            child.spawn().unwrap().wait().unwrap();

            assert_eq!(
                true,
                String::from_utf8_lossy(&out.stdout).contains("The current local time is:")
            );
        }
    }
}
