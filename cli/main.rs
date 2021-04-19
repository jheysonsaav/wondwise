use clap::App;

fn main() {
    const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const APP_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

    let matches = App::new("wondwise")
        .version(APP_VERSION)
        .about(APP_DESCRIPTION)
        .get_matches();
}
