use crate::error::Status;
use crate::internal::*;
use std::ffi::CString;

use crate::definition::{DefinitionVisitor, DefinitionVisitorMultiplexer};
use crate::print_defs::PrintingDefinitionVisitor;

type ReaderHandle = OwnedExternHandle<OTF2_Reader_struct, OTF2_ErrorCode>;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum ReaderError {
    #[error("null byte at position {1} in the {0} argument")]
    NullByte(&'static str, usize),
    #[error("OTF2_Reader_Open failed for path '{0}'")]
    OpenReturnedNull(String),
    #[error("unknown error occurred")]
    Unknown,
}

#[derive(Debug)]
pub struct Reader {
    handle: ReaderHandle,
    anchor_file: CString,
}

impl Reader {
    pub fn open(anchor_file: String) -> Result<Self, ReaderError> {
        let anchor_file: CString = CString::new(anchor_file)
            .map_err(|e| ReaderError::NullByte("anchor_file", e.nul_position()))?;
        let handle = unsafe { OTF2_Reader_Open(anchor_file.as_ptr()) };
        if handle.is_null() {
            return Err(ReaderError::OpenReturnedNull(
                anchor_file.to_string_lossy().into_owned(),
            ));
        }
        let mut handle = OwnedExternHandle::new(handle, OTF2_Reader_Close);
        Reader::set_serial_collective_callbacks(&mut handle).map_err(|_| ReaderError::Unknown)?;
        Reader::read_all_global_definitions(&mut handle).map_err(|_| ReaderError::Unknown)?;
        Ok(Self {
            handle,
            anchor_file,
        })
    }

    fn set_serial_collective_callbacks(handle: &mut ReaderHandle) -> Result<(), Status> {
        unsafe { OTF2_Reader_SetSerialCollectiveCallbacks(handle.as_raw_mut()) }.into()
    }

    fn read_all_global_definitions(handle: &mut ReaderHandle) -> Result<(), Status> {
        let mut global_callbacks = OwnedExternHandle::new(
            unsafe { OTF2_GlobalDefReaderCallbacks_New() },
            OTF2_GlobalDefReaderCallbacks_Delete,
        );
        dbg!(&global_callbacks);
        let mut def_visitor = DefinitionVisitorMultiplexer::new();
        def_visitor.add_visitor(Box::new(PrintingDefinitionVisitor::new()));
        def_visitor.set_global_def_reader_callbacks(&mut global_callbacks)?;
        let global_def_reader = unsafe { OTF2_Reader_GetGlobalDefReader(handle.as_raw_mut()) };
        assert!(
            !global_def_reader.is_null(),
            "OTF2_Reader_GetGlobalDefReader returned null"
        );
        unsafe {
            OTF2_Reader_RegisterGlobalDefCallbacks(
                handle.as_raw_mut(),
                global_def_reader,
                global_callbacks.as_raw_mut(),
                &def_visitor as *const _ as *mut _,
            )
        }?;
        let mut definitions_read: u64 = 0;
        unsafe {
            OTF2_Reader_ReadAllGlobalDefinitions(
                handle.as_raw_mut(),
                global_def_reader,
                &mut definitions_read,
            )
        }?;
        unsafe { OTF2_Reader_CloseGlobalDefReader(handle.as_raw_mut(), global_def_reader) }?;
        eprintln!("<{definitions_read} definitions were read>");
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_reader() {
        let reader = Reader::open("/home/adam/Dropbox/Durham-RA/experiments/bots-strassen/trace/serial_512.15132/serial_512.15132.otf2".to_string());
        dbg!(&reader);
    }
}
