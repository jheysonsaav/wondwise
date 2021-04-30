// Copyright (C) 2021 Wondwise Authors. All rights reserved. GPL-3.0 license.

mod commands;

use clap::{App, Arg, Shell, SubCommand};
use std::{env, io, thread};
use wondwise_utils::{
    dirs::WondwiseDirs,
    logs::{Log, LogLevel},
};

fn main() {
    thread::spawn(|| {
        WondwiseDirs::load().verify();
    });

    let app = App::new("wondwise")
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("log-level")
                .short("L")
                .long("log-level")
                .default_value("info")
                .possible_values(&["debug", "info"])
                .takes_value(true)
                .global(true),
        )
        .subcommands(commands::commands())
        .subcommand(
            SubCommand::with_name("completions")
                .about("Output shell completion script to standard output")
                .arg(
                    Arg::with_name("shell")
                        .possible_values(&["bash", "zsh", "powershell", "fish", "elvish"])
                        .required(true),
                ),
        );

    let matches = app.clone().get_matches();

    env::set_var("LOG_LEVEL", matches.value_of("log-level").unwrap());

    match matches.subcommand() {
        ("start", Some(args)) => commands::start::setup(args),
        ("completions", Some(args)) => {
            let mut shell: Shell = Shell::Bash;

            match args.value_of("shell") {
                Some("bash") => shell = Shell::Bash,
                Some("zsh") => shell = Shell::Zsh,
                Some("elvish") => shell = Shell::Elvish,
                Some("powershell") => shell = Shell::PowerShell,
                Some("fish") => shell = Shell::Fish,
                None | Some(_) => Log::new(LogLevel::Error, 1, "Cannot get shell").show(),
            }

            app.clone()
                .gen_completions_to("wondwise", shell, &mut io::stdout())
        }
        _ => {
            app.clone().print_help().expect("Cannot print help message");
            println!();
        }
    }
}
