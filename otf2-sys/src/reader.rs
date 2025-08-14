//! Safe wrappers around low-level OTF2_Reader operations.
//! 
//! Provides safe interface for OTF2_Reader operations. Wraps low-level return values like
//! `OTF2_ErrorCode` into more expressive `Result` types. Encapsulates raw pointers in handles.

use crate::internal::*;
use crate::error::{Status, StatusCode};
use crate::event::{EventVisitor, GlobalEvtReaderCallbacks};
use crate::definition::{Definitions, GlobalDefReaderCallbacks, DefinitionVisitor, LocationRegistry};
use std::ffi::{CString, CStr};
use std::ops::ControlFlow;

use OTF2_ErrorCode::*;

struct LocalEvtFiles<'r> {
    reader: &'r mut Reader,
}

impl<'r> LocalEvtFiles<'r> {
    fn open(reader: &'r mut Reader) -> Status<Self> {
        unsafe { OTF2_Reader_OpenEvtFiles(reader.handle.as_mut_ptr()) }?;
        Ok(LocalEvtFiles { reader })
    }

    fn prepare_evt_files(&mut self, locations: &[OTF2_LocationRef]) {
        // NOTE: apparently this call is required before reading any events. Assume the pointer is
        // cached in the reader somewhere and cleaned up by OTF2...
        for &location in locations {
            unsafe { OTF2_Reader_GetEvtReader(self.reader.handle.as_mut_ptr(), location) };
        }
    }
}

impl core::ops::Drop for LocalEvtFiles<'_> {
    fn drop(&mut self) {
        let _ = unsafe { OTF2_Reader_CloseEvtFiles(self.reader.handle.as_mut_ptr()) };
    }
}

struct LocalDefFiles<'r> {
    reader: &'r mut Reader,
}

impl<'r> LocalDefFiles<'r> {
    fn open(reader: &'r mut Reader) -> Status<Self> {
        unsafe { OTF2_Reader_OpenDefFiles(reader.handle.as_mut_ptr()) }?;
        Ok(LocalDefFiles { reader })
    }

    fn read_local_definitions(self, locations: &[OTF2_LocationRef]) -> Status<u64> {
        let mut definitions_read: u64 = 0;
        for &location in locations {
            definitions_read += LocalDefReader::new(self.reader, location).read_definitions()?;
        }
        Ok(definitions_read)
    }
}

impl core::ops::Drop for LocalDefFiles<'_> {
    fn drop(&mut self) {
        let _ = unsafe { OTF2_Reader_CloseDefFiles(self.reader.handle.as_mut_ptr()) };
    }
}

struct LocalDefReader<'r> {
    reader: &'r mut Reader,
    handle: Handle<OTF2_DefReader>,
}

impl core::ops::Drop for LocalDefReader<'_> {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            let _ = unsafe { OTF2_Reader_CloseDefReader(self.reader.handle.as_mut_ptr(), self.handle.as_mut_ptr()) };
        }
    }
}

impl<'r> LocalDefReader<'r> {
    pub fn new(reader: &'r mut Reader, location: OTF2_LocationRef) -> Self {
        let handle = Handle::from_raw(unsafe { OTF2_Reader_GetDefReader(reader.handle.as_mut_ptr(), location) })
            .expect("failed to get local def reader");
        LocalDefReader { reader, handle }
    }

    pub fn read_definitions(mut self) -> Status<u64> {
        let mut definitions_read = 0;
        unsafe {
            OTF2_Reader_ReadAllLocalDefinitions(
                self.reader.handle.as_mut_ptr(),
                self.handle.as_mut_ptr(),
                &mut definitions_read,
            )?;
        }
        Ok(definitions_read)
    }
}

#[derive(Debug)]
struct GlobalEvtReader<'r> {
    reader: &'r mut Reader,
    handle: Handle<OTF2_GlobalEvtReader>,
}

impl core::ops::Drop for GlobalEvtReader<'_> {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            let _ = unsafe { OTF2_Reader_CloseGlobalEvtReader(self.reader.handle.as_mut_ptr(), self.handle.as_mut_ptr()) };
        }
    }
}

impl<'r> GlobalEvtReader<'r> {
    fn new(reader: &'r mut Reader) -> Self {
        let handle = Handle::from_raw(unsafe { OTF2_Reader_GetGlobalEvtReader(reader.handle.as_mut_ptr()) })
            .expect("failed to get global evt reader");
        GlobalEvtReader { reader, handle }
    }

    pub fn read_events(&mut self, visitors: Vec<&mut dyn EventVisitor>, batch_size: std::num::NonZeroU64) -> Status<u64> {
        let mut total_events = 0;
        let callbacks = GlobalEvtReaderCallbacks::new()?;
        unsafe { OTF2_GlobalEvtReader_SetCallbacks(self.handle.as_mut_ptr(), callbacks.as_ptr(), &visitors as *const _ as *mut _) }?;
        loop {
            match self.read_next_event_batch(batch_size)? {
                ControlFlow::Continue(events_read) => {
                    total_events += events_read;
                }
                ControlFlow::Break(events_read) => {
                    total_events += events_read;
                    break
                }
            }
        }
        Ok(total_events)
    }

    fn read_next_event_batch(&mut self, batch_size: std::num::NonZeroU64) -> Status<ControlFlow<u64, u64>> {
        let mut events_read = 0;
        unsafe { OTF2_GlobalEvtReader_ReadEvents(self.handle.as_mut_ptr(), batch_size.get(), &mut events_read) }?;
        if events_read < batch_size.get() {
            Ok(ControlFlow::Break(events_read))
        } else {
            Ok(ControlFlow::Continue(events_read))
        }
    }
}

#[derive(Debug)]
struct Reader {
    handle: Handle<OTF2_Reader>,
    global_def_reader_handle: Handle<OTF2_GlobalDefReader>,
}

impl core::ops::Deref for Reader {
    type Target = OTF2_Reader;

    fn deref(&self) -> &Self::Target {
        self.handle.as_ref()
    }
}

impl core::ops::DerefMut for Reader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.handle.as_mut()
    }
}

impl core::ops::Drop for Reader {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            let reader = self.handle.take();
            let global_def_reader = self.global_def_reader_handle.take();
            unsafe {
                let _ = OTF2_Reader_CloseGlobalDefReader(reader, global_def_reader);
                let _ = OTF2_Reader_Close(reader);
            }
        }
    }
}

impl Reader {
    /// Open a new OTF2 reader from the given anchor file and read all definitions.
    pub fn open(anchor_file: CString) -> Status<Self> {
        let mut reader = Reader::create_handles(&anchor_file).ok_or(StatusCode::from_raw(OTF2_ERROR_MEM_ALLOC_FAILED))?;
        reader.set_serial_collective_callbacks()?;
        let mut locations = LocationRegistry::new();
        let mut callbacks = GlobalDefReaderCallbacks::new()?;
        reader.read_global_definitions(vec![&mut locations], &mut callbacks)?;
        let locations: Vec<_> = locations.into_keys().collect();
        reader.select_locations(&locations)?;
        reader.read_local_definitions(&locations)?;
        reader.prepare_evt_files(&locations)?;
        Ok(reader)
    }

    pub fn get_global_evt_reader(&'_ mut self) -> GlobalEvtReader<'_> {
        GlobalEvtReader::new(self)
    }

    fn create_handles(anchor_file: &CStr) -> Option<Self> {
        let mut handle = Handle::from_raw(unsafe { OTF2_Reader_Open(anchor_file.as_ptr()) })?;
        let global_def_reader_handle = Handle::from_raw(unsafe { OTF2_Reader_GetGlobalDefReader(handle.as_mut_ptr()) })?;
        Some(Reader { handle, global_def_reader_handle })
    }

    pub fn get_global_definitions(&mut self, visitors: Vec<&mut dyn DefinitionVisitor>) -> Status<u64> {
        let mut callbacks = GlobalDefReaderCallbacks::new()?;
        self.read_global_definitions(visitors, &mut callbacks)
    }

    fn read_global_definitions(&mut self, mut visitors: Vec<&mut dyn DefinitionVisitor>, callbacks: &mut GlobalDefReaderCallbacks) -> Status<u64> {
        let mut definitions_read: u64 = 0;
        unsafe {
            OTF2_Reader_RegisterGlobalDefCallbacks(
                self.handle.as_mut_ptr(),
                self.global_def_reader_handle.as_mut_ptr(),
                callbacks.as_mut_ptr(),
                &mut visitors as *const _ as *mut _,
            )?;
            OTF2_Reader_ReadAllGlobalDefinitions(
                self.handle.as_mut_ptr(),
                self.global_def_reader_handle.as_mut_ptr(),
                &mut definitions_read,
            )?;
        }
        Ok(definitions_read)
    }

    fn read_local_definitions(&mut self, locations: &[OTF2_LocationRef]) -> Status<u64> {
        LocalDefFiles::open(self)?.read_local_definitions(locations)
    }

    fn prepare_evt_files(&mut self, locations: &[OTF2_LocationRef]) -> Status<()> {
        LocalEvtFiles::open(self)?.prepare_evt_files(locations);
        Ok(())
    }

    fn select_locations(&mut self, locations: &[OTF2_LocationRef]) -> Status<()> {
        for &location in locations {
            unsafe { OTF2_Reader_SelectLocation(self.handle.as_mut_ptr(), location) }?;
        }
        Ok(())
    }

    fn set_serial_collective_callbacks(&mut self) -> Status<()> {
        unsafe { OTF2_Reader_SetSerialCollectiveCallbacks(self.handle.as_mut_ptr()).into() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::event::PrintingEventVisitor;

    #[test]
    fn test_reader() {
        let anchor_file = CString::new("/home/adam/Dropbox/Durham-RA/experiments/bots-strassen/trace/serial_512.15132/serial_512.15132.otf2").expect("Failed to create CString");
        let reader = Reader::open(anchor_file);
        dbg!(&reader);
        let mut reader = reader.unwrap();
        let mut evt_printer = PrintingEventVisitor;
        let mut global_evt_reader = reader.get_global_evt_reader();
        global_evt_reader.read_events(vec![&mut evt_printer], std::num::NonZeroU64::new(100).unwrap()).expect("Failed to read events");
        dbg!(&global_evt_reader);
    }
}
