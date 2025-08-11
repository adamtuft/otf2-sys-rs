//! Fetch and build the OTF2 source

/*
- Use fetch_source to get the source
- Detect whether it's built
- configure
- make
- report result
*/

// pub fn fetch_otf2_source(manifest_dir: &std::path::Path, out_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
//     let sources = fetch_source::load_sources(&manifest_dir)?;
//     let otf2 = sources.get("otf2::3.0").expect("Should have otf2::3.0 in sources table");
//     dbg!(&otf2);
//     println!("fetch OTF2 into {out_path:?}");
//     Ok(())
// }
