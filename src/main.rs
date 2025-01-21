use cbr_alarm::cli;
use clap::Parser;
use color_eyre::Result;

use cbr_alarm::app;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Cli::parse();

    let mut tm_s = Duration::from_secs(5);
    if let Some(cmd) = args.cmd {
        tm_s = match cmd {
            cli::Commands::Timeout(t) => t.parse().unwrap(),
        };
    }

    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = app::App::new(tm_s).run(terminal);
    ratatui::restore();
    app_result
}
