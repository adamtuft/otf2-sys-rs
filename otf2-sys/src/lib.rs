#![feature(try_trait_v2)]
#![allow(unused_imports)]
#![allow(dead_code)]

// mod archive;
mod c;
mod definition;
mod error;
mod event;
mod handle;
mod reader;

mod internal {
    pub use super::c::*;
    pub use super::handle::*;
}
