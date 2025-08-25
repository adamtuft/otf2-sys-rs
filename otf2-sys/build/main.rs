use std::path::PathBuf;
use anyhow::{Error, Context};

mod bindgen;
mod build_otf2;

use build_otf2::build_from_source;

fn detect_source_cache_dir() -> Result<Option<PathBuf>, Error> {
    let cache_dir = match std::env::var_os("CARGO_FETCH_SOURCE_CACHE") {
        Some(dir) => PathBuf::from(dir),
        None => {
            let project_dirs = directories::ProjectDirs::from("", "", "cargo-fetch-source").context("could not determine cache directory")?;
            project_dirs.cache_dir().to_path_buf()
        }
    };
    Ok(Some(cache_dir).filter(|d| fetch_source::Cache::cache_file_exists(d)))
}

fn main() -> Result<(), Error> {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    dbg!(&out_path);
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    dbg!(&manifest_dir);
    let mut sources = fetch_source::load_sources(&manifest_dir)?;
    let otf2 = sources
        .remove("otf2::3.0")
        .expect("Should have otf2::3.0 in sources table");
    println!("fetch OTF2 into {out_path:?}");
    dbg!(&otf2);
    let otf2_install_dir = match detect_source_cache_dir()? {
        Some(dir) => {
            let cache = fetch_source::Cache::read(&dir).context(format!("failed to read fetch-source cache {dir:?}"))?;
            assert!(cache.items().contains(&otf2), "expected otf2 to be cached");
            let artefact_dir = cache.cache_dir().append(cache.items().relative_path(&otf2));
            let install_dir = artefact_dir.join("install");
            assert!(install_dir.is_dir(), "expected a cached install directory");
            install_dir
        },
        None => {
            let build_dir = out_path.join("otf2-build");
            let install_dir = out_path.join("otf2-install");
            let artefact = otf2.fetch(&build_dir).context("failed to fetch OTF2 source")?;
            dbg!(&artefact);
            let src_dir: PathBuf = artefact.path().join("otf2-3.0");
            build_from_source(&src_dir, &install_dir)?
        },
    };
    dbg!(&otf2_install_dir);
    bindgen::generate(&otf2_install_dir, &out_path)
}
