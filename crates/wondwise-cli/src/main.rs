mod commands;

use clap::App;
use commands::{Command, StartCommand};

fn main() {
    let matches = App::new("wondwise")
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(StartCommand::command())
        .get_matches();

    match matches.subcommand() {
        ("start", Some(args)) => StartCommand::setup(args),
        _ => StartCommand::setup(&matches),
    }
}
