use std::path::PathBuf;
use anyhow::Error;

mod bindgen;
mod build_otf2;

use directories::ProjectDirs;
use fetch_source::{Cache, load_sources};
use build_otf2::build_from_source;

macro_rules! load_from_cache {
    ($dir:ident, $src:ident) => {{
        let cache = Cache::read(&$dir).expect(&format!("failed to read fetch-source cache {:?}", $dir));
        assert!(cache.items().contains(&$src), concat!("expected ", stringify!($src), " to be cached"));
        cache.cached_path(&$src)
    }};
}

macro_rules! expect_cached_install {
    ($dir:ident, $src:ident) => {{
        let install_dir = load_from_cache!($dir, $src).join("install");
        assert!(install_dir.is_dir(), "install directory not found: {install_dir:?}");
        install_dir
    }};
}

macro_rules! fetch_build_install {
    ($src:ident, $src_dir:expr, $fetch_dir:expr, $install_dir:expr) => {{
        let artefact = $src.fetch($fetch_dir).expect("failed to fetch source");
        build_from_source(
            artefact.path().join($src_dir),
            $install_dir
        )
    }};
}

/// Check for a given source cache directory. Check `CARGO_FETCH_SOURCE_CACHE` then fall back to the
/// default user cache directory.
macro_rules! get_source_cache_dir {
    ($env_var:expr, $cache_dir:expr) => {{
        match std::env::var_os($env_var) {
            Some(dir) => PathBuf::from(dir),
            None => {
                let project_dirs = ProjectDirs::from("", "", $cache_dir).expect("home directory couldn't be detected");
                project_dirs.cache_dir().to_path_buf()
            }
        }
    }}
}

fn main() -> Result<(), Error> {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let otf2 = load_sources(&manifest_dir).expect("failed to load sources from manifest file")
        .remove("otf2::3.0").expect("Should have otf2::3.0 in sources table");
    let otf2_install_dir = {
        let cache_dir = get_source_cache_dir!("CARGO_FETCH_SOURCE_CACHE", "cargo-fetch-source");
        if Cache::cache_file_exists(&cache_dir) {
            expect_cached_install!(cache_dir, otf2)
        } else {
            fetch_build_install!(
                otf2,
                "otf2-3.0",
                out_path.join("otf2-build"),
                out_path.join("otf2-install")
            )?
        }
    };
    bindgen::generate(&otf2_install_dir, &out_path)
}
