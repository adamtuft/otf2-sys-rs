use std::path::PathBuf;

mod bindgen;
mod otf2_ext;

fn detect_source_cache_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    match std::env::var_os("CARGO_FETCH_SOURCE_CACHE") {
        Some(dir) => Ok(PathBuf::from(dir)),
        None => {
            let project_dirs = directories::ProjectDirs::from("", "", "cargo-fetch-source")
                .ok_or("could not determine cache directory".to_string())?;
            Ok(project_dirs.cache_dir().to_path_buf())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let source_cache_dir = detect_source_cache_dir()?;
    let cache = fetch_source::Cache::read(&source_cache_dir)?;
    dbg!(&out_path);
    dbg!(&manifest_dir);
    dbg!(&source_cache_dir);
    dbg!(&cache);
    let sources = fetch_source::load_sources(&manifest_dir)?;
    let otf2 = sources
        .get("otf2::3.0")
        .expect("Should have otf2::3.0 in sources table");
    println!("fetch OTF2 into {out_path:?}");
    if cache.items().contains(otf2) {
        println!("OTF2 cached");
        let artefact_dir = cache.cache_dir().append(cache.items().relative_path(otf2));
        let install_dir = artefact_dir.join("install");
        assert!(install_dir.is_dir(), "expected a cached install directory");
        bindgen::generate(&install_dir, &out_path)?;
        Ok(())
    } else {
        panic!("OTF2 not cached");
    }
}
