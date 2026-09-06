use blawdioh_render::{assets::load_assets, render_frame, save_image};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Parser)]
struct Cli {
    #[arg(required = true)]
    path: PathBuf,

    #[arg(short, long, default_value = "24.0")]
    fps: f64,
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

    let assets_path = Path::new("./assets");
    let frames_dir = Path::new("./temp_frames");
    fs::create_dir_all(frames_dir).expect("Failed to create output directory for frames");

    let asset_bundle = load_assets(&assets_path).expect("Failed to load assets");

    println!("Rendering...");

    let progress_bar = ProgressBar::new(total as u64);
    progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} frames ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );

    if total > 0 {
        for (i, keyframe) in keyframes.iter().enumerate() {
            let frame = render_frame(&asset_bundle, keyframe);
            save_image(&frame, &frames_dir.join(format!("{}.png", i)))
                .unwrap_or_else(|err| panic!("Failed to save frame {}: {:?}", i, err));

            progress_bar.inc(1);
        }
    }

    progress_bar.finish_with_message("Rendering complete!");
}
