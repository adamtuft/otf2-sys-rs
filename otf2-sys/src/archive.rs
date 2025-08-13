use crate::internal::*;
use crate::{
    error::{Status, StatusResult},
    file,
};
use std::ffi::CString;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum ArchiveBuilderError {
    #[error("{0} is a required argument to OTF2_Archive_Open")]
    MissingField(&'static str),
    #[error("null byte at position {pos} in the {field} argument")]
    NullByte { field: &'static str, pos: usize },
    #[error("OTF2_Archive_Open returned null")]
    OpenReturnedNull { name: String, path: String },
}

impl ArchiveBuilderError {
    pub fn null_byte(field: &'static str, pos: usize) -> Self {
        ArchiveBuilderError::NullByte { field, pos }
    }

    pub fn open_returned_null(name: String, path: String) -> Self {
        ArchiveBuilderError::OpenReturnedNull { name, path }
    }
}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum ArchiveError {
    #[error("OTF2_Archive_Close returned error: {0}")]
    Close(Status),
}

type ArchiveHandle = OwnedExternHandle<OTF2_Archive_struct, OTF2_ErrorCode>;

mod private {
    use super::*;

    #[derive(Builder, Debug)]
    #[builder(derive(Debug))]
    #[builder(build_fn(skip, error = "ArchiveBuilderError"))]
    #[builder(name = "ArchiveBuilder")]
    pub struct ArchiveOpenArgs {
        #[builder(private)]
        mode: file::Mode,
        chunk_size_events: u64,
        chunk_size_defs: u64,
        substrate: file::Substrate,
        compression: file::Compression,
    }

    impl ArchiveBuilder {
        pub fn read(&mut self) -> &mut Self {
            self.mode(file::Mode::OTF2_FILEMODE_READ)
        }

        pub fn write(&mut self) -> &mut Self {
            self.mode(file::Mode::OTF2_FILEMODE_WRITE)
        }

        pub fn readwrite(&mut self) -> &mut Self {
            self.mode(file::Mode::OTF2_FILEMODE_MODIFY)
        }

        pub fn open(self, path: String, name: String) -> Result<Archive, ArchiveBuilderError> {
            let path: CString = CString::new(path)
                .map_err(|e| ArchiveBuilderError::null_byte("path", e.nul_position()))?;
            let name: CString = CString::new(name)
                .map_err(|e| ArchiveBuilderError::null_byte("name", e.nul_position()))?;
            let handle = unsafe {
                OTF2_Archive_Open(
                    path.as_ptr(),
                    name.as_ptr(),
                    self.mode.ok_or(ArchiveBuilderError::MissingField("mode"))? as u8,
                    self.chunk_size_events
                        .unwrap_or(OTF2_CHUNK_SIZE_EVENTS_DEFAULT as u64),
                    self.chunk_size_defs
                        .unwrap_or(OTF2_CHUNK_SIZE_DEFINITIONS_DEFAULT as u64),
                    self.substrate
                        .unwrap_or(file::Substrate::OTF2_SUBSTRATE_POSIX) as u8,
                    self.compression
                        .unwrap_or(file::Compression::OTF2_COMPRESSION_NONE)
                        as u8,
                )
            };
            if handle.is_null() {
                Err(ArchiveBuilderError::open_returned_null(
                    name.to_string_lossy().to_string(),
                    path.to_string_lossy().to_string(),
                ))
            } else {
                Ok(Archive {
                    handle: ArchiveHandle::new(handle, OTF2_Archive_Close),
                    path,
                    name,
                })
            }
        }
    }
}

pub use private::ArchiveBuilder;

#[derive(Debug)]
pub struct Archive {
    handle: ArchiveHandle,
    path: CString,
    name: CString,
}

impl Archive {
    pub fn close(mut self) -> Result<(), ArchiveError> {
        self.handle.drop_handle().map_err(ArchiveError::Close)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_archive_builder() {
        let mut builder = ArchiveBuilder::default();
        builder.read();
        dbg!(&builder);
        let archive = builder.open(
            "/home/adam/Dropbox/Durham-RA/experiments/bots-strassen/trace/serial_512.15132"
                .to_string(),
            "serial_512.15132".to_string(),
        );
        dbg!(&archive);
        assert!(archive.is_ok());
        let closed = archive.unwrap().close();
        dbg!(&closed);
        assert!(closed.is_ok());
    }
}
