mod commands;

extern crate libc;
extern crate rustbox;
extern crate rinput;

use std::io::stdin;
use std::error::Error;
use std::default::Default;

use rinput::{
    Input
};

// (Full example with detailed comments in examples/01d_quick_example.rs)
//
// This example demonstrates clap's full 'custom derive' style of creating arguments which is the
// simplest method of use, but sacrifices some flexibility.
use orbtk::prelude::*;
use clap::Clap;
use rustbox::{InitOptions, Color, RustBox, InputMode, OutputMode};
use rustbox::Key;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
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
        SubCommand::UI(t) => {
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

    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Hello, world!");
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black,
                  "Press 'q' to quit.");
    rustbox.present();
    // loop {
    //     match rustbox.poll_event(false) {
    //         Ok(rustbox::Event::KeyEvent(key)) => {
    //             match key {
    //                 Key::Char('q') => { break; }
    //                 _ => {}
    //             }
    //         }
    //         Err(e) => panic!("{}", e.description()),
    //         _ => {}
    //     }
    // }
}

fn start_ui() {
    Application::new()
        .window(|ctx| {
            Window::new()
                .title("OrbTk - minimal example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(TextBlock::new().text("OrbTk")
                    .build(ctx))
                .build(ctx)
        }).run();
}
