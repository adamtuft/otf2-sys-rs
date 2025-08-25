use std::path::{Path, PathBuf};
use anyhow::{Error, Context};
use autotools::Config;

pub fn build_from_source<P, Q>(src_dir: P, install_dir: Q) -> Result<PathBuf, Error>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    if !install_dir.as_ref().is_dir() {
        std::fs::create_dir_all(&install_dir.as_ref()).context("failed to create install directory")?;
    }
    let mut config = Config::new(src_dir.as_ref());
    let dst = config
        .env("PYTHON", ":")
        .out_dir(install_dir.as_ref())
        .enable_static()
        .disable_shared()
        .build();
    Ok(dst)
}
