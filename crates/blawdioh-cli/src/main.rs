use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(required = true)]
    path: PathBuf,

    #[arg(short, long, default_value = "24.0")]
    fps: f64,
}

fn main() {
    let cli = Cli::parse();

    let keyframes = blawdioh_engine::generate_keyframes(&cli.path, cli.fps);
    println!("{}", keyframes.len());
}
