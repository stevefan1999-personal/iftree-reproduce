use std::{path::PathBuf, env};
use eyre::{eyre, Result};
use fs_extra::dir::CopyOptions;

fn main() -> Result<()> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let dir_copy_opt = CopyOptions::new()
        .overwrite(true)
        .skip_exist(true)
        .copy_inside(true)
        .content_only(true);

    let merged_dir = out_dir.join("merged");
    fs_extra::dir::copy(
        &manifest_dir.join("example_data1"),
        &merged_dir,
        &dir_copy_opt,
    )
    .map_err(|e| eyre!(e))?;

    fs_extra::dir::copy(
        &manifest_dir.join("example_data2"),
        &merged_dir,
        &dir_copy_opt,
    )
    .map_err(|e| eyre!(e))?;
    Ok(())
}