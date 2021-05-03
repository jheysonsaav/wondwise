use clap::{crate_description, crate_version, App};
use rustyline::{error::ReadlineError, Editor};

fn main() {
    // Create clap application
    let _matches = App::new("wondwise")
        .version(crate_version!())
        .about(crate_description!())
        .get_matches();

    // Create and start prompt with rustyline
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
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
    rl.save_history("history.txt").unwrap();
}
