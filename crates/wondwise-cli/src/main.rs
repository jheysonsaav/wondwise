mod commands;

use clap::App;
use commands::{Command, StartCommand};

fn main() {
    let app = App::new("wondwise")
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(StartCommand::command());

    let matches = app.clone().get_matches();

    match matches.subcommand() {
        ("start", Some(args)) => StartCommand::setup(args),
        _ => {
            app.clone().print_help().expect("Cannot print help message");
            println!();
        }
    }
}
