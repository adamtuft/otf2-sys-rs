//! Safe wrappers around low-level OTF2_Reader operations.
//! 
//! Provides safe interface for OTF2_Reader operations. Wraps low-level return values like
//! `OTF2_ErrorCode` into more expressive `Result` types. Encapsulates raw pointers in handles.

use crate::internal::*;
use crate::error::{Status, StatusCode};
use crate::event::{Event, EventKind, GlobalEvtReaderCallbacks};
use crate::definition::{GlobalDefReaderCallbacks, Definition, DefinitionVisitor, LocationRegistry, DefinitionList};
use std::ffi::{CString, CStr};
use std::ops::ControlFlow;
use std::collections::VecDeque;

use OTF2_ErrorCode::*;

struct LocalEvtFiles<'r> {
    reader: &'r mut Trace,
}

impl core::ops::Drop for LocalEvtFiles<'_> {
    fn drop(&mut self) {
        let _ = unsafe { OTF2_Reader_CloseEvtFiles(self.reader.handle.as_mut_ptr()) };
    }
}

impl<'r> LocalEvtFiles<'r> {
    fn open(reader: &'r mut Trace) -> Status<Self> {
        unsafe { OTF2_Reader_OpenEvtFiles(reader.handle.as_mut_ptr()) }?;
        Ok(LocalEvtFiles { reader })
    }

    fn select_local_evt_readers(&mut self, locations: &[OTF2_LocationRef]) {
        // NOTE: apparently this call is required before reading any events AND before creating a global event reader. Assume the pointer is cached in the reader somewhere and cleaned up by OTF2...
        for &location in locations {
            unsafe { OTF2_Reader_GetEvtReader(self.reader.handle.as_mut_ptr(), location) };
        }
    }
}

struct LocalDefFiles<'r> {
    reader: &'r mut Trace,
}

impl core::ops::Drop for LocalDefFiles<'_> {
    fn drop(&mut self) {
        let _ = unsafe { OTF2_Reader_CloseDefFiles(self.reader.handle.as_mut_ptr()) };
    }
}

impl<'r> LocalDefFiles<'r> {
    fn open(reader: &'r mut Trace) -> Status<Self> {
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
    reader: &'r mut Trace,
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
    pub fn new(reader: &'r mut Trace, location: OTF2_LocationRef) -> Self {
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
    reader: &'r mut Trace,
}

impl<'r> core::ops::Drop for GlobalDefReader<'r> {
    fn drop(&mut self) {
        unsafe {
            let _ = OTF2_Reader_CloseGlobalDefReader(self.reader.handle.as_mut_ptr(), self.handle.as_mut_ptr());
        }
    }
}

impl<'r> GlobalDefReader<'r> {
    fn new(reader: &'r mut Trace) -> Status<Self> {
        let handle = Handle::from_raw(unsafe { OTF2_Reader_GetGlobalDefReader(reader.handle.as_mut_ptr()) }).ok_or(StatusCode::from_raw(OTF2_ERROR_MEM_ALLOC_FAILED))?;
        Ok(GlobalDefReader { handle, reader })
    }

    fn read_global_definitions(mut self, callbacks: &mut GlobalDefReaderCallbacks) -> Status<(u64, DefinitionList)> {
        let mut definitions_read: u64 = 0;
        let mut definitions = DefinitionList::new();
        unsafe {
            OTF2_Reader_RegisterGlobalDefCallbacks(
                self.reader.handle.as_mut_ptr(),
                self.handle.as_mut_ptr(),
                callbacks.as_mut_ptr(),
                &mut definitions as *const _ as *mut _,
            )?;
            OTF2_Reader_ReadAllGlobalDefinitions(
                self.reader.handle.as_mut_ptr(),
                self.handle.as_mut_ptr(),
                &mut definitions_read,
            )?;
        }
        Ok((definitions_read, definitions))
    }
}

#[derive(Debug)]
pub struct Trace {
    handle: Handle<OTF2_Reader>,
}

impl core::ops::Drop for Trace {
    fn drop(&mut self) {
        unsafe {
            let _ = OTF2_Reader_Close(self.handle.take());
        }
    }
}

impl Trace {
    pub fn read_definitions(&mut self) -> Status<(u64, DefinitionList)> {
        let mut callbacks = GlobalDefReaderCallbacks::new()?;
        GlobalDefReader::new(self)?.read_global_definitions(&mut callbacks)
    }

    pub fn get_event_reader<'r>(&'r mut self, batch_size: u64) -> Status<EventReader<'r>> {
        let (_, definitions) = self.read_definitions()?;
        let locations = definitions.iter()
            .filter_map(|def| match def {
                Definition::Location { defn, value: _ } => Some(*defn),
                _ => None,
            })
            .collect();
        self.get_local_event_reader(locations, batch_size)
    }

    pub fn get_local_event_reader<'r>(&'r mut self, locations: Vec<OTF2_LocationRef>, batch_size: u64) -> Status<EventReader<'r>> {
        EventReader::new(self, locations, batch_size)
    }

    pub fn iter_events<'r>(&'r mut self, batch_size: u64) -> Status<EventIter<'r>> {
        self.get_event_reader(batch_size)?.into_event_iter()
    }
}

pub fn open(anchor_file: CString) -> Status<Trace> {
    let mut handle = Handle::from_raw(unsafe { OTF2_Reader_Open(anchor_file.as_ptr()) }).ok_or(StatusCode::from_raw(OTF2_ERROR_MEM_ALLOC_FAILED))?;
    unsafe { OTF2_Reader_SetSerialCollectiveCallbacks(handle.as_mut_ptr())}?;
    Ok(Trace { handle })
}

#[derive(Debug)]
pub struct EventReader<'r> {
    reader: &'r mut Trace,
    handle: Handle<OTF2_GlobalEvtReader>,
    locations: Vec<OTF2_LocationRef>,
    batch_size: u64,
}

impl<'r> core::ops::Drop for EventReader<'r> {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                let _ = OTF2_Reader_CloseGlobalEvtReader(self.reader.handle.as_mut_ptr(), self.handle.take());
            }
        }
    }
}

impl<'r> EventReader<'r> {

    pub fn new(reader: &'r mut Trace, locations: Vec<OTF2_LocationRef>, batch_size: u64) -> Status<Self> {
        for location in &locations {
            unsafe { OTF2_Reader_SelectLocation(reader.handle.as_mut_ptr(), *location) }?;
        }
        LocalDefFiles::open(reader)?.read_local_definitions(&locations)?;
        LocalEvtFiles::open(reader)?.select_local_evt_readers(&locations);
        let global_evt_reader = Handle::from_raw(unsafe { OTF2_Reader_GetGlobalEvtReader(reader.handle.as_mut_ptr()) })
            .expect("failed to get global evt reader");
        Ok(EventReader { reader, locations, batch_size, handle: global_evt_reader })
    }

    pub fn into_event_iter(self) -> Status<EventIter<'r>> {
        let batch_size = self.batch_size;
        EventIter::new(self, batch_size)
    }
}

#[derive(Debug)]
pub struct EventIter<'r> {
    evt_reader: EventReader<'r>,
    batch_size: u64,
    // The event queue is boxed to give it a persistent location in memory. Its address is passed
    // to OTF2_GlobalEvtReader_SetCallbacks during `new()` then used during `read_next_event_batch`
    event_queue: Box<VecDeque<Event>>,
}

impl<'r> std::iter::Iterator for EventIter<'r> {
    type Item = Status<Event>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.event_queue.is_empty() {
            if let Err(code) = self.read_next_event_batch() {
                return Some(Err(code))
            }
        }
        self.event_queue.pop_front().map(Status::Ok)
    }
}

impl<'r> EventIter<'r> {
    fn new(mut evt_reader: EventReader<'r>, batch_size: u64) -> Status<Self> {
        let mut event_queue = Box::new(VecDeque::with_capacity(batch_size as usize));
        let callbacks = GlobalEvtReaderCallbacks::new()?;
        unsafe { OTF2_GlobalEvtReader_SetCallbacks(evt_reader.handle.as_mut_ptr(), callbacks.as_ptr(), event_queue.as_mut() as *const _ as *mut _) }?;
        Ok(EventIter { evt_reader, batch_size, event_queue })
    }

    fn read_next_event_batch(&mut self) -> Status<u64> {
        let mut events_read = 0;
        unsafe { OTF2_GlobalEvtReader_ReadEvents(self.evt_reader.handle.as_mut_ptr(), self.batch_size, &mut events_read) }?;
        Ok(events_read)
    }
}

#[cfg(test)]
mod test {
    use core::num;

    use super::*;

    use crate::definition::Definition;

    fn count_events(event_iter: EventIter) -> usize {
        event_iter.count()
    }

    #[test]
    fn test_reader() {
        let anchor_file = CString::new("/home/adam/Dropbox/Durham-RA/experiments/bots-strassen/trace/serial_512.15132/serial_512.15132.otf2").expect("Failed to create CString");
        let mut trace = open(anchor_file).unwrap();
        for event in trace.iter_events(1000).expect("Failed to get event iter") {
            match event {
                Ok(event) => { println!("Event: {}", event.as_json()); },
                Err(err) => { eprintln!("Error reading event: {err}"); break; },
            }
        }
    }
}
