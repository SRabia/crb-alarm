use cbr_alarm::cli;
use clap::Parser;
use color_eyre::Result;

use cbr_alarm::app;
use cbr_alarm::spoty;
use std::time::Duration;

fn main() -> Result<()> {
    let args = cli::Cli::parse();

    let mut tm_s = Duration::from_secs(5);
    if let Some(cmd) = args.cmd {
        tm_s = match cmd {
            cli::Commands::Timeout(t) => t.parse().unwrap(),
        };
    }
    let spoty = spoty::SpotiApi::new();

    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = app::App::new(tm_s, spoty).run(terminal);
    ratatui::restore();
    app_result
}
