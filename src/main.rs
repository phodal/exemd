use std::fs;

use clap::Clap;
use colored::*;

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
    Run(EditorCmd),
}

#[derive(Clap)]
struct EditorCmd {
    path: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Run(t) => {
            run_markdown(t);
        }
    }
}

fn run_markdown(args: EditorCmd) {
    let filename = args.path;
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut parser = rmd::Rmd::new(contents);
    let vec = parser.parse();

    for cmd in vec.into_iter() {
        match execute_command(cmd) {
            Ok(status) => match status.code() {
                Some(code) => {
                    // todo: try catch error for not broken
                    std::process::exit(code)
                },
                None => return,
            },
            Err(err) => {
                eprintln!("{} {}", "ERROR:".red(), err);
                std::process::exit(1)
            }
        }
    }
}
