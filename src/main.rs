extern crate libc;
extern crate rinput;

use std::fs;
use std::io::stdin;

use clap::Clap;
use colored::*;

use rinput::{Editor, Input};
use rinput::rustbox::rustbox::{InitOptions, InputMode, OutputMode, RustBox};

use crate::rmd::executor::execute_command;

mod rmd;

#[derive(Clap)]
struct Opts {
    config: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    UI(EditorCmd),

    Box(EditorCmd),

    Run(EditorCmd),
}

#[derive(Clap)]
struct EditorCmd {
    path: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::UI(_t) => {
            start_ui();
        }
        SubCommand::Box(t) => {
            start_box(t);
        }
        SubCommand::Run(t) => {
            run_markdown(t);
        }
    }
}

fn is_atty(fileno: libc::c_int) -> bool {
    // FIXME: find a way to do this without unsafe
    //        std::io doesn't allow for this, currently
    unsafe { libc::isatty(fileno) != 0 }
}

fn run_markdown(args: EditorCmd) {
    let filename = args.path;
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut parser = rmd::Rmd::new(contents);
    let vec = parser.parse();

    for cmd in vec.into_iter() {
        match execute_command(cmd) {
            Ok(status) => match status.code() {
                Some(code) => std::process::exit(code),
                None => return,
            },
            Err(err) => {
                eprintln!("{} {}", "ERROR:".red(), err);
                std::process::exit(1)
            }
        }
    }
}

fn start_box(args: EditorCmd) {
    let stdin_is_atty = is_atty(libc::STDIN_FILENO);
    let stderr_is_atty = is_atty(libc::STDERR_FILENO);

    // editor source - either a filename or stdin
    let source = if stdin_is_atty {
        Input::Filename(Some(args.path))
    } else {
        Input::Stdin(stdin())
    };

    // initialise rustbox
    let rustbox = match RustBox::init(InitOptions {
        buffer_stderr: stderr_is_atty,
        input_mode: InputMode::Esc,
        output_mode: OutputMode::EightBit,
    }) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let mut editor = Editor::new(source, rustbox);
    editor.start();
}

fn start_ui() {}
