use blawdioh_engine::types::KeyFrame;
use image::{ImageResult, RgbaImage};
use std::path::Path;

use crate::assets::AssetBundle;

pub mod assets;

pub fn render_frame(assets: &AssetBundle, keyframe: &KeyFrame) -> RgbaImage {
    let mut frame = assets.base_texture.clone();

    if let Some(mouth_sprite) = assets.mouths.get(&keyframe.mouth.state) {
        image::imageops::overlay(
            &mut frame,
            mouth_sprite,
            keyframe.mouth.offset_x as i64,
            keyframe.mouth.offset_y as i64,
        );
    }

    if let Some(eye_sprite) = assets.eyes.get(&keyframe.eye.state) {
        image::imageops::overlay(
            &mut frame,
            eye_sprite,
            keyframe.eye.offset_x as i64,
            keyframe.eye.offset_y as i64,
        );
    }

    return frame;
}

pub fn save_image(image: &RgbaImage, path: &Path) -> ImageResult<()> {
    image.save(path)
}
