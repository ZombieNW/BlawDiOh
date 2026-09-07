use blawdioh_engine::types::{EyeState, MouthState};
use image::RgbaImage;
use std::{collections::HashMap, error::Error, path::Path};

#[derive(Debug)]
pub struct AssetBundle {
    pub base_texture: RgbaImage,
    pub mouths: HashMap<MouthState, RgbaImage>,
    pub eyes: HashMap<EyeState, RgbaImage>,
}

pub fn load_assets(dir: &Path) -> Result<AssetBundle, Box<dyn Error>> {
    let base_texture = image::open(dir.join("base.png"))
        .expect("Base texture not found")
        .to_rgba8();

    // load/build mouth texture hashmap
    let mut mouths = HashMap::new();
    let mouth_dir = dir.join("mouths");
    for state in MouthState::ALL {
        let path = mouth_dir.join(format!("{}.png", state.filename_stem()));
        let texture = image::open(&path)
            .expect("Mouth texture not found")
            .to_rgba8();
        mouths.insert(state, texture);
    }

    // load/build eye texture hashmap
    let mut eyes = HashMap::new();
    let eye_dir = dir.join("eyes");
    for state in EyeState::ALL {
        let path = eye_dir.join(format!("{}.png", state.filename_stem()));
        let texture = image::open(&path)
            .expect("Mouth texture not found")
            .to_rgba8();
        eyes.insert(state, texture);
    }

    Ok(AssetBundle {
        base_texture,
        mouths,
        eyes,
    })
}
