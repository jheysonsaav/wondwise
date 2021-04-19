use super::Command;
use clap::{App, ArgMatches, SubCommand};
pub struct StartCommand;

impl<'a, 'b> Command<'a, 'b> for StartCommand {
    fn command() -> App<'a, 'b> {
        SubCommand::with_name("start").about("Create new wondwise session")
    }

    fn setup(_args: &ArgMatches) {
        println!("start session!");
    }
}
