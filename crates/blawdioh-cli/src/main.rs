use blawdioh_render::assets::load_assets;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::{env, path::PathBuf};

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

    // Get proper paths for everything
    let exe_dir = env::current_exe()
        .expect("Failed to get executable path")
        .parent()
        .expect("Failed to get executable directory")
        .to_path_buf();
    let assets_dir = exe_dir.join("assets");
    let output_path = &cli.output;

    println!("Generating audio keyframes...");
    let keyframes = blawdioh_engine::generate_keyframes(&cli.path, cli.fps);
    println!("Total keyframes: {}", keyframes.len());

    if keyframes.is_empty() {
        return;
    }

    println!("Loading assets...");
    let assets = load_assets(&assets_dir).expect("Failed to load assets");

    println!("Rendering...");
    let progress_bar = ProgressBar::new(keyframes.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:40.cyan/blue}] {pos}/{len} frames ({eta} remaining)")
            .unwrap()
            .progress_chars("█░ "),
    );

    blawdioh_render::render_video(
        &assets,
        &keyframes,
        &cli.path,
        &output_path,
        cli.fps,
        || progress_bar.inc(1),
    )
    .expect("Failed to render video");

    progress_bar.finish_with_message("Rendering complete!");
    println!("Saved video to {}", output_path.display());
}
