use super::Command;
use clap::{App, ArgMatches, SubCommand};
use std::{thread, time::Duration};

pub struct StartCommand;

impl<'a, 'b> Command<'a, 'b> for StartCommand {
    fn command() -> App<'a, 'b> {
        SubCommand::with_name("start").about("Create new wondwise session")
    }

    fn setup(_args: &ArgMatches) {
        println!("Starting session");

        for i in 1..6 {
            thread::sleep(Duration::from_millis(50));
            println!("Starting... {}", i);
        }

        println!("Finished session");
    }
}
