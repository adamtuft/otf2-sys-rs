//! Generate bindings to OTF2
use std::path::Path;

use bindgen::builder;

pub fn generate(
    install_dir: &Path,
    out_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let install_dir = install_dir.display();
    println!("cargo::rustc-link-search={install_dir}/lib");
    println!("cargo::rustc-link-lib=otf2");

    let bindings = builder()
        .header(format!("{install_dir}/include/otf2/otf2.h"))
        .rustified_enum("OTF2_ErrorCode")
        .rustified_enum("OTF2_CallbackCode_enum")
        .rustified_enum("OTF2_FileMode_enum")
        .rustified_enum("OTF2_FileSubstrate_enum")
        .rustified_enum("OTF2_Compression_enum")
        .must_use_type("OTF2_ErrorCode")
        .generate_cstr(true)
        .clang_args([format!("-I{install_dir}/include")])
        .generate()?;

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}
