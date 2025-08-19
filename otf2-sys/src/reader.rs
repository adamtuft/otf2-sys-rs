//! Safe wrappers around low-level OTF2_Reader operations.
//! 
//! Provides safe interface for OTF2_Reader operations. Wraps low-level return values like
//! `OTF2_ErrorCode` into more expressive `Result` types. Encapsulates raw pointers in handles.

use crate::internal::*;
use crate::error::{Status, StatusCode};
use crate::event::{Event, EventKind, EventVisitor, GlobalEvtReaderCallbacks};
use crate::definition::{GlobalDefReaderCallbacks, Definition, DefinitionVisitor, LocationRegistry, DefinitionList};
use std::ffi::{CString, CStr};
use std::ops::ControlFlow;
use std::collections::VecDeque;

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
    reader: &'r mut Reader,
}

impl core::ops::Drop for LocalEvtFiles<'_> {
    fn drop(&mut self) {
        let _ = unsafe { OTF2_Reader_CloseEvtFiles(self.reader.handle.as_mut_ptr()) };
    }
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

struct LocalDefFiles<'r> {
    reader: &'r mut Reader,
}

impl core::ops::Drop for LocalDefFiles<'_> {
    fn drop(&mut self) {
        let _ = unsafe { OTF2_Reader_CloseDefFiles(self.reader.handle.as_mut_ptr()) };
    }
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
struct GlobalDefReader<'r> {
    handle: Handle<OTF2_GlobalDefReader>,
    reader: &'r mut Reader,
}

impl<'r> core::ops::Drop for GlobalDefReader<'r> {
    fn drop(&mut self) {
        unsafe {
            let _ = OTF2_Reader_CloseGlobalDefReader(self.reader.handle.as_mut_ptr(), self.handle.as_mut_ptr());
        }
    }
}

impl<'r> GlobalDefReader<'r> {
    fn new(reader: &'r mut Reader) -> Status<Self> {
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
pub struct Reader {
    handle: Handle<OTF2_Reader>,
}

impl core::ops::Drop for Reader {
    fn drop(&mut self) {
        unsafe {
            let _ = OTF2_Reader_Close(self.handle.take());
        }
    }
}

impl Reader {
    pub fn read_definitions(&mut self) -> Status<(u64, DefinitionList)> {
        let mut callbacks = GlobalDefReaderCallbacks::new()?;
        GlobalDefReader::new(self)?.read_global_definitions(&mut callbacks)
    }

    pub fn event_reader<'r>(&'r mut self, batch_size: u64) -> Status<EventReader<'r>> {
        let (_, definitions) = self.read_definitions()?;
        let locations = definitions.iter()
            .filter_map(|def| match def {
                Definition::Location { defn, value: _ } => Some(*defn),
                _ => None,
            })
            .collect();
        self.local_event_reader(locations, batch_size)
    }

    pub fn local_event_reader<'r>(&'r mut self, locations: Vec<OTF2_LocationRef>, batch_size: u64) -> Status<EventReader<'r>> {
        EventReader::new(self, locations, batch_size)
    }
}

pub fn open(anchor_file: CString) -> Status<Reader> {
    let mut handle = Handle::from_raw(unsafe { OTF2_Reader_Open(anchor_file.as_ptr()) }).ok_or(StatusCode::from_raw(OTF2_ERROR_MEM_ALLOC_FAILED))?;
    unsafe { OTF2_Reader_SetSerialCollectiveCallbacks(handle.as_mut_ptr())}?;
    Ok(Reader { handle })
}

#[derive(Debug)]
pub struct EventReader<'r> {
    reader: &'r mut Reader,
    locations: Vec<OTF2_LocationRef>,
    batch_size: u64,
}

impl<'longer> EventReader<'longer> {

    pub fn new(reader: &'longer mut Reader, locations: Vec<OTF2_LocationRef>, batch_size: u64) -> Status<Self> {
        let mut this = EventReader { reader, locations, batch_size };
        this.select_locations()?;
        this.read_local_definitions()?;
        this.prepare_evt_files()?;
        Ok(this)
    }

    pub fn events<'shorter>(&'shorter mut self) -> Status<EventIter<'shorter, 'longer>> {
        EventIter::new(self, self.batch_size)
    }

    fn select_locations(&mut self) -> Status<()> {
        for &location in &self.locations {
            unsafe { OTF2_Reader_SelectLocation(self.reader.handle.as_mut_ptr(), location) }?;
        }
        Ok(())
    }

    fn read_local_definitions(&mut self) -> Status<()> {
        LocalDefFiles::open(&mut self.reader)?.read_local_definitions(&self.locations)?;
        Ok(())
    }

    fn prepare_evt_files(&mut self) -> Status<()> {
        LocalEvtFiles::open(&mut self.reader)?.prepare_evt_files(&self.locations);
        Ok(())
    }
}

#[derive(Debug)]
pub struct EventIter<'shorter, 'longer>
where
    'longer: 'shorter,
{
    handle: Handle<OTF2_GlobalEvtReader>,
    reader: &'shorter mut EventReader<'longer>,
    batch_size: u64,
    // The event queue is boxed to give it a persistent location in memory. Its address is passed
    // to OTF2_GlobalEvtReader_SetCallbacks during `new()` then used during `read_next_event_batch`
    event_queue: Box<VecDeque<Event>>,
}

impl<'shorter, 'longer> core::ops::Drop for EventIter<'shorter, 'longer>
where
    'longer: 'shorter,
{
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                let _ = OTF2_Reader_CloseGlobalEvtReader(self.reader.reader.handle.as_mut_ptr(), self.handle.take());
            }
        }
    }
}

impl<'shorter, 'longer> std::iter::Iterator for EventIter<'shorter, 'longer>
where
    'longer: 'shorter,
{
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        if self.event_queue.is_empty() {
            let _ = self.read_next_event_batch();
        }
        self.event_queue.pop_front()
    }
}

impl<'shorter, 'longer> EventIter<'shorter, 'longer>
where
    'longer: 'shorter,
{
    fn new(reader: &'shorter mut EventReader<'longer>, batch_size: u64) -> Status<Self> {
        let handle = Handle::from_raw(unsafe { OTF2_Reader_GetGlobalEvtReader(reader.reader.handle.as_mut_ptr()) })
            .expect("failed to get global evt reader");
        let mut event_iter = EventIter { reader, handle, batch_size, event_queue: Box::new(VecDeque::with_capacity(batch_size as usize)) };
        event_iter.set_callbacks()?;
        Ok(event_iter)
    }

    fn set_callbacks(&mut self) -> Status<()> {
        let callbacks = GlobalEvtReaderCallbacks::new()?;
        unsafe { OTF2_GlobalEvtReader_SetCallbacks(self.handle.as_mut_ptr(), callbacks.as_ptr(), self.event_queue.as_mut() as *const _ as *mut _) }?;
        Ok(())
    }

    fn read_next_event_batch(&mut self) -> Status<u64> {
        let mut events_read = 0;
        unsafe { OTF2_GlobalEvtReader_ReadEvents(self.handle.as_mut_ptr(), self.batch_size, &mut events_read) }?;
        Ok(events_read)
    }
}

#[cfg(test)]
mod test {
    use core::num;

    use super::*;

    use crate::definition::Definition;
    use crate::event::PrintingEventVisitor;

    fn count_events(event_iter: EventIter) -> usize {
        event_iter.count()
    }

    #[test]
    fn test_reader() {
        let anchor_file = CString::new("/home/adam/Dropbox/Durham-RA/experiments/bots-strassen/trace/serial_512.15132/serial_512.15132.otf2").expect("Failed to create CString");
        let mut reader = open(anchor_file).unwrap();
        let mut event_reader = reader.event_reader(1000).expect("Failed to get global event reader");
        let num_events = count_events(event_reader.events().expect("Failed to get events"));
        dbg!(num_events);
        let num_events = count_events(event_reader.events().expect("Failed to get events"));
        dbg!(num_events);
    }
}
