struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("No args given");
    let path: String = std::env::args().nth(2).expect("No path given");

    let args = Cli {
        pattern,
        path: std::path::PathBuf::from(path),
    };

    print!("pattern {:?} {:?}", args.pattern, args.path);
}
