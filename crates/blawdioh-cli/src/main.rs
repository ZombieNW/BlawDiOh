use blawdioh_render::{assets::load_assets, render_frame, save_image};
use clap::Parser;
use std::path::{Path, PathBuf, absolute};

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

    let assets_path = Path::new("./assets");
    let absolute_assets = absolute(assets_path).unwrap();
    println!("{}", absolute_assets.display());

    let parent_dir = Path::new("./");
    let absolute_parent = absolute(parent_dir).unwrap();
    println!("{}", absolute_parent.display());

    let asset_bundle = load_assets(&assets_path).unwrap();

    if total > 0 {
        for (i, keyframe) in keyframes.iter().enumerate() {
            if i % 100 == 0 {
                let _ = save_image(&render_frame(&asset_bundle, keyframe), parent_dir);
                println!("Keyframe #{}: {:?}", i, keyframe);
            }
        }
    }
}
