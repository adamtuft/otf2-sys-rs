#![feature(try_trait_v2)]
#![allow(unused_imports)]
#![allow(dead_code)]

mod archive;
mod c;
mod definition;
mod error;
mod event;
mod handle;
mod print_defs;
mod reader;

pub mod file {
    use super::c;
    pub type Mode = c::OTF2_FileMode_enum;
    pub type Substrate = c::OTF2_FileSubstrate_enum;
    pub type Compression = c::OTF2_Compression_enum;
}

mod internal {
    pub use derive_builder::Builder;
    // pub use super::error::IntoStatusResult;
    pub use super::c::*;
    pub use super::handle::*;
}
