//! Generate bindings to OTF2
use std::path::Path;

use bindgen::{EnumVariation, builder};

pub fn generate(
    install_dir: &Path,
    out_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let install_dir = install_dir.display();
    println!("cargo::rustc-link-search={install_dir}/lib");
    println!("cargo::rustc-link-lib=otf2");

    let bindings = builder()
        .header(format!("{install_dir}/include/otf2/otf2.h"))
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: false,
        })
        .must_use_type("OTF2_ErrorCode")
        .new_type_alias("OTF2_.*.Ref")
        .new_type_alias("OTF2_.*.Type")
        .new_type_alias("OTF2_Type")
        .new_type_alias("OTF2_AttributeValue")
        .derive_eq(true)
        .derive_ord(true)
        .no_partialeq("OTF2_.*.Callback[s]{0,1}")
        .generate_cstr(true)
        .clang_args([format!("-I{install_dir}/include")])
        .generate()?;

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}
