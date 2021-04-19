mod commands;

use clap::App;
use commands::{Command, StartCommand};

fn main() {
    const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const APP_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

    let matches = App::new("wondwise")
        .version(APP_VERSION)
        .about(APP_DESCRIPTION)
        .subcommand(StartCommand::command())
        .get_matches();

    match matches.subcommand() {
        ("start", Some(args)) => StartCommand::setup(args),
        _ => StartCommand::setup(&matches),
    }
}
