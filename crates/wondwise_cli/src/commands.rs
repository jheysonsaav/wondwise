pub mod start;

use clap::{App, SubCommand};

pub fn commands() -> Vec<App<'static, 'static>> {
    vec![SubCommand::with_name("start").about("Create new Wondwise Shell session")]
}
