use blawdioh_render::assets::load_assets;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};

#[derive(Parser)]
struct Cli {
    #[arg(required = true)]
    path: PathBuf,

    #[arg(short, long, default_value = "24.0")]
    fps: f64,

    #[arg(short, long, default_value = "output.mp4")]
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    println!("Generating audio keyframes...");
    let keyframes = blawdioh_engine::generate_keyframes(&cli.path, cli.fps);
    let total = keyframes.len();

    println!("Total keyframes: {}", total);
    if total == 0 {
        return;
    }

    println!("Loading assets...");
    let assets = load_assets(Path::new("./assets")).expect("Failed to load assets");

    println!("Rendering...");
    let progress_bar = ProgressBar::new(total as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:40.cyan/blue}] {pos}/{len} frames ({eta} remaining)")
            .unwrap()
            .progress_chars("█░ "),
    );

    blawdioh_render::render_video(&assets, &keyframes, &cli.path, &cli.output, cli.fps, || {
        progress_bar.inc(1)
    })
    .expect("Failed to render video");

    progress_bar.finish_with_message("Rendering complete!");
    println!("Saved video to {}", cli.output.display());
}
