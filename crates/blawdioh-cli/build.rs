use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_all(&entry.path(), &dest_path)?;
        } else {
            fs::copy(entry.path(), &dest_path)?;
        }
    }

    Ok(())
}

/// Walks from OUT_DIR to profile dir
fn find_target_profile_dir() -> PathBuf {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    for ancestor in out_dir.ancestors() {
        let is_direct_child_of_target = ancestor
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            == Some("target");

        let is_build_script_dir = ancestor.file_name().and_then(|n| n.to_str()) == Some("build");

        if is_direct_child_of_target && !is_build_script_dir {
            return ancestor.to_path_buf();
        }
    }

    // Plan B
    out_dir.ancestors().nth(3).unwrap().to_path_buf()
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let src_assets = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("assets"))
        .filter(|p| p.exists());

    let Some(src_assets) = src_assets else {
        println!("cargo:warning=Assets directory not found");
        return;
    };

    println!("cargo:rerun-if-changed={}", src_assets.display());

    let target_dir = find_target_profile_dir();
    let dest_assets = target_dir.join("assets");

    if let Err(e) = copy_dir_all(&src_assets, &dest_assets) {
        println!("cargo:warning=Failed to copy assets: {e}");
    }
}
