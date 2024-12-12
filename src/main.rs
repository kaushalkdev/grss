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

    info!("Starting counter with value {}", counter);
    warn!("Some random warning");

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
    find_matches(&content?, &args.pattern, std::io::stdout());
    Ok(())
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    debug!("in find matches fn");
    for line in content.lines() {
        if line.contains(pattern) {
            let result = writeln!(writer, "{}", line);

            match result {
                Ok(_) => {
                    debug!("Write success full")
                }
                Err(error) => {
                    error!("Failed to write : {error:?}")
                }
            }
        }
    }
    debug!("find matches complete");
}

#[test]
fn test_find_match() {
    let mut res_vec = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut res_vec);
    assert_eq!(res_vec, b"lorem ipsum\n");
}
