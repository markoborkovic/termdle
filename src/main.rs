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
    let debug_mode = Args::parse().debug;

    let mut terminal = ui::init().unwrap();
    let mut app = app::App::new(debug_mode);
    app.run(&mut terminal).unwrap();
    ui::restore().unwrap();
}
