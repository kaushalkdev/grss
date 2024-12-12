use anyhow::{Context, Result};
use clap::Parser;
use log::{debug, error, info, trace, warn};
use std::thread;
use std::time::Duration;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for.
    pattern: String,
    /// The path to the file to read.
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Cli::parse();
    let counter = 5;
    trace!("Some random trace value");
    debug!("Debugging values");
    info!("Starting counter with value {}", counter);
    warn!("Some random warning");
    error!("Some random error");

    let pb = indicatif::ProgressBar::new(counter);

    println!(
        "############# Reading file {} ################",
        args.path.display()
    );

    for _ in 0..counter {
        pb.inc(1);
        thread::sleep(Duration::from_secs(1));
    }

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()));
    for line in content?.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
