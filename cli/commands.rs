mod start;

use clap::{App, ArgMatches};
pub use start::StartCommand;

pub trait Command<'a, 'b> {
    fn command() -> App<'a, 'b>;
    fn setup(args: &ArgMatches);
}
