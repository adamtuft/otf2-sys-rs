//! Generate bindings to OTF2
use std::path::Path;

use bindgen::{EnumVariation, builder, callbacks::ParseCallbacks};

#[derive(Debug)]
struct Parser;

impl ParseCallbacks for Parser {
    fn add_derives(&self, info: &bindgen::callbacks::DeriveInfo<'_>) -> Vec<String> {
        let mut derives: Vec<&'static str> = Vec::new();
        self.check_derive_serde(info, &mut derives);
        dbg!(info);
        derives.into_iter().map(|s| s.to_string()).collect()
    }
}

impl Parser {
    fn check_derive_serde(&self, info: &bindgen::callbacks::DeriveInfo<'_>, derives: &mut Vec<&'static str>) {
        if self.item_is_ref_type(info) || self.item_is_type_enum_wrapper(info) {
            derives.push("serde::Serialize");
            derives.push("serde::Deserialize");
        }
    }

    fn item_is_ref_type(&self, info: &bindgen::callbacks::DeriveInfo<'_>) -> bool {
        info.name.starts_with("OTF2_") && info.name.ends_with("Ref")
    }

    fn item_is_type_enum_wrapper(&self, info: &bindgen::callbacks::DeriveInfo<'_>) -> bool {
        info.name.starts_with("OTF2_") && info.name.ends_with("Type")
    }
}

pub fn generate(
    install_dir: &Path,
    out_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let install_dir = install_dir.display();
    println!("cargo::rustc-link-search={install_dir}/lib");
    println!("cargo::rustc-link-lib=otf2");

    let bindings = builder()
        .header(format!("{install_dir}/include/otf2/otf2.h"))
        .parse_callbacks(Box::new(Parser))
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: false,
        })
        .must_use_type("OTF2_ErrorCode")
        .new_type_alias("OTF2_.*.Ref")
        .new_type_alias("OTF2_.*.Type")
        .new_type_alias("OTF2_Type")
        .new_type_alias("OTF2_AttributeValue")
        .new_type_alias("OTF2_MetricValue")
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
