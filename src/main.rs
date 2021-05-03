mod utils;

use crate::utils::logs::{Log, LogLevel};
use clap::{crate_description, crate_version, App, Arg};
use rustyline::{error::ReadlineError, Editor};
use std::env;
use utils::dirs::WondwiseDirs;

fn main() {
    // Create clap application
    let matches = App::new("wondwise")
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("log-level")
                .short("L")
                .long("log-level")
                .default_value("info")
                .possible_values(&["info", "debug"])
                .takes_value(true)
                .global(true),
        )
        .get_matches();

    env::set_var("LOG_LEVEL", matches.value_of("log-level").unwrap());

    // Create and start prompt with rustyline
    let mut rl = Editor::<()>::new();

    #[cfg(unix)]
    let history_file = format!(
        "{}/history.yml",
        WondwiseDirs::load().data_dir().to_str().unwrap()
    );

    #[cfg(windows)]
    let history_file = format!(
        "{}\\history.yml",
        WondwiseDirs::load().data_dir().as_str().unwrap()
    );

    if rl.load_history(history_file.as_str()).is_err() {
        Log::new(LogLevel::Debug, 2, "No previous history").show();
    }
    loop {
        let readline = rl.readline("-| ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                continue;
            }
        }
    }
    rl.save_history(history_file.as_str()).unwrap();
}
