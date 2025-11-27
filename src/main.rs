mod app;
mod ui;
mod words;

use clap::Parser;

/// A simple terminal version of the Worlde game
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Start Termdle in debug mode
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    app::App::new(Args::parse().debug)
        .run(&mut ui::init().unwrap())
        .unwrap();
    ui::restore().unwrap();
}
