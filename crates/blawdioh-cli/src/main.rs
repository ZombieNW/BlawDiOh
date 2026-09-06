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
    let total = keyframes.len();

    println!("Total keyframes: {}", total);

    if total > 0 {
        for (i, keyframe) in keyframes.iter().enumerate() {
            if i % 100 == 0 {
                println!("Keyframe #{}: {:?}", i, keyframe);
            }
        }
    }
}
