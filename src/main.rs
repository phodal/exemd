extern crate libc;
extern crate rustbox;
extern crate rinput;

use std::io::stdin;
use std::error::Error;
use std::default::Default;

use rinput::{Input, Editor};

use clap::Clap;
use rustbox::{InitOptions, Color, RustBox, InputMode, OutputMode};
use rustbox::Key;

#[derive(Clap)]
struct Opts {
    input: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version = "0.0.1")]
    UI(EditorCmd),
    Box(EditorCmd),
}

/// A subcommand for controlling testing
#[derive(Clap)]
struct EditorCmd {
    /// Some input. Because this isn't an Option<T> it's required to be used
    #[clap(short, long)]
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
    }
}

fn is_atty(fileno: libc::c_int) -> bool {
    // FIXME: find a way to do this without unsafe
    //        std::io doesn't allow for this, currently
    unsafe { libc::isatty(fileno) != 0 }
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

fn start_ui() {

}
