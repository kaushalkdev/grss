use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for.
    pattern: String,
    /// The path to the file to read.
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .expect("\n ===========> INCORRECT FILE FORMAT <==========\n");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
