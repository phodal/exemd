extern crate libc;
extern crate rinput;

use std::io::stdin;
use clap::Clap;

use rinput::{Editor, Input};
use rinput::rustbox::rustbox::{RustBox, InitOptions, InputMode, OutputMode};

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

fn start_ui() {}
