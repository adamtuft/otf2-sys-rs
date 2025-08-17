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

macro_rules! declare_type_states {
    ($($state_name:ident),*) => {
        mod private {
            pub trait Sealed {}
        }

        mod states {
            use super::private;
            pub trait State: private::Sealed {}
            $(
                #[derive(Debug, Clone, Copy)]
                pub struct $state_name;
                impl private::Sealed for $state_name {}
                impl State for $state_name {}
            )*
        }
    }
}

declare_type_states!(Open, Readable);

struct LocalEvtFiles<'r> {
    reader: &'r mut ReadyReader,
}

impl core::ops::Drop for LocalEvtFiles<'_> {
    fn drop(&mut self) {
        let _ = unsafe { OTF2_Reader_CloseEvtFiles(self.reader.handle.as_mut_ptr()) };
    }
}

impl<'r> LocalEvtFiles<'r> {
    fn open(reader: &'r mut ReadyReader) -> Status<Self> {
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

struct LocalDefFiles<'r> {
    reader: &'r mut ReadyReader,
}

impl core::ops::Drop for LocalDefFiles<'_> {
    fn drop(&mut self) {
        let _ = unsafe { OTF2_Reader_CloseDefFiles(self.reader.handle.as_mut_ptr()) };
    }
}

impl<'r> LocalDefFiles<'r> {
    fn open(reader: &'r mut ReadyReader) -> Status<Self> {
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

struct LocalDefReader<'r> {
    reader: &'r mut ReadyReader,
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
    pub fn new(reader: &'r mut ReadyReader, location: OTF2_LocationRef) -> Self {
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
struct GlobalDefReader<'r> {
    handle: Handle<OTF2_GlobalDefReader>,
    reader: &'r mut OpenReader,
}

impl<'r> core::ops::Drop for GlobalDefReader<'r> {
    fn drop(&mut self) {
        unsafe {
            let _ = OTF2_Reader_CloseGlobalDefReader(self.reader.handle.as_mut_ptr(), self.handle.as_mut_ptr());
        }
    }
}

impl<'r> GlobalDefReader<'r> {
    fn new(reader: &'r mut OpenReader) -> Status<Self> {
        let handle = Handle::from_raw(unsafe { OTF2_Reader_GetGlobalDefReader(reader.handle.as_mut_ptr()) }).ok_or(StatusCode::from_raw(OTF2_ERROR_MEM_ALLOC_FAILED))?;
        Ok(GlobalDefReader { handle, reader })
    }

    fn read_global_definitions(mut self, def_visitor: &mut impl DefinitionVisitor, callbacks: &mut GlobalDefReaderCallbacks) -> Status<u64> {
        let mut definitions_read: u64 = 0;
        unsafe {
            OTF2_Reader_RegisterGlobalDefCallbacks(
                self.reader.handle.as_mut_ptr(),
                self.handle.as_mut_ptr(),
                callbacks.as_mut_ptr(),
                def_visitor as *const _ as *mut _,
            )?;
            OTF2_Reader_ReadAllGlobalDefinitions(
                self.reader.handle.as_mut_ptr(),
                self.handle.as_mut_ptr(),
                &mut definitions_read,
            )?;
        }
        Ok(definitions_read)
    }
}

#[derive(Debug)]
pub struct Reader<S: states::State> {
    handle: Handle<OTF2_Reader>,
    _state: std::marker::PhantomData<S>,
}

type OpenReader = Reader<states::Open>;
type ReadyReader = Reader<states::Readable>;

impl<S: states::State> core::ops::Drop for Reader<S> {
    fn drop(&mut self) {
        unsafe {
            let _ = OTF2_Reader_Close(self.handle.take());
        }
    }
}

pub fn open(anchor_file: CString) -> Status<OpenReader> {
    let mut handle = Handle::from_raw(unsafe { OTF2_Reader_Open(anchor_file.as_ptr()) }).ok_or(StatusCode::from_raw(OTF2_ERROR_MEM_ALLOC_FAILED))?;
    unsafe { OTF2_Reader_SetSerialCollectiveCallbacks(handle.as_mut_ptr())}?;
    Ok(OpenReader {
        handle,
        _state: std::marker::PhantomData,
    })
}

impl OpenReader {
    pub fn get_global_definitions(mut self, def_visitor: &mut impl DefinitionVisitor, num_definitions: &mut u64) -> Status<ReadyReader> {
        let mut callbacks = GlobalDefReaderCallbacks::new()?;
        *num_definitions = GlobalDefReader::new(&mut self)?.read_global_definitions(def_visitor, &mut callbacks)?;
        Ok(ReadyReader { handle: Handle::from_raw_unchecked(self.handle.take()), _state: std::marker::PhantomData })
    }
}

#[derive(Debug)]
pub struct GlobalEvtReader {
    handle: Handle<OTF2_GlobalEvtReader>,
    reader: ReadyReader,
}

impl core::ops::Drop for GlobalEvtReader {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                let _ = OTF2_Reader_CloseGlobalEvtReader(self.reader.handle.as_mut_ptr(), self.handle.take());
            }
        }
    }
}

impl GlobalEvtReader {
    fn new(mut reader: ReadyReader) -> Self {
        let handle = Handle::from_raw(unsafe { OTF2_Reader_GetGlobalEvtReader(reader.handle.as_mut_ptr()) })
            .expect("failed to get global evt reader");
        GlobalEvtReader { reader, handle }
    }

    pub fn read_events(&mut self, visitor: &mut dyn EventVisitor, batch_size: std::num::NonZeroU64) -> Status<u64> {
        let mut total_events = 0;
        let callbacks = GlobalEvtReaderCallbacks::new()?;
        unsafe { OTF2_GlobalEvtReader_SetCallbacks(self.handle.as_mut_ptr(), callbacks.as_ptr(), visitor as *const _ as *mut _) }?;
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

impl ReadyReader {

    /// Consumes the `Reader` to produce a `GlobalEvtReader` for reading all events from the trace
    pub fn into_global_evt_reader(mut self, locations: &[OTF2_LocationRef]) -> Status<GlobalEvtReader> {
        self.select_locations(locations)?;
        LocalDefFiles::open(&mut self)?.read_local_definitions(locations)?;
        LocalEvtFiles::open(&mut self)?.prepare_evt_files(locations);
        Ok(GlobalEvtReader::new(self))
    }

    fn select_locations(&mut self, locations: &[OTF2_LocationRef]) -> Status<()> {
        for &location in locations {
            unsafe { OTF2_Reader_SelectLocation(self.handle.as_mut_ptr(), location) }?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use core::num;

    use super::*;

    use crate::event::PrintingEventVisitor;

    #[test]
    fn test_reader() {
        let anchor_file = CString::new("/home/adam/Dropbox/Durham-RA/experiments/bots-strassen/trace/serial_512.15132/serial_512.15132.otf2").expect("Failed to create CString");
        let reader = open(anchor_file);
        dbg!(&reader);
        let mut reader = reader.unwrap();
        let mut locations = LocationRegistry::new();
        let mut num_definitions = 0;
        let mut reader = reader.get_global_definitions(&mut locations, &mut num_definitions).expect("Failed to get global definitions");
        let location_refs = locations.keys().cloned().collect::<Vec<_>>();
        let mut global_evt_reader = reader.into_global_evt_reader(&location_refs).expect("Failed to get global event reader");
        let mut evt_printer = PrintingEventVisitor;
        global_evt_reader.read_events(&mut evt_printer, std::num::NonZeroU64::new(1000).unwrap()).expect("Failed to read events");
        dbg!(&global_evt_reader);
    }
}
