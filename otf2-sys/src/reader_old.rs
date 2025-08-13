use crate::error::Status;
use crate::internal::*;
use std::ffi::CString;

use crate::definition::{DefinitionVisitor, DefinitionVisitorMultiplexer, StringRegistry, AttributeRegistry, LocationRegistry};
use crate::event::{EventVisitorMultiplexer, PrintingEventVisitor};
use crate::print_defs::PrintingDefinitionVisitor;

type ReaderHandle = OwnedExternHandle<OTF2_Reader_struct, OTF2_ErrorCode>;
type GlobalEvtReaderCallbacks = OwnedExternHandle<OTF2_GlobalEvtReaderCallbacks_struct, ()>;

impl ReaderHandle {
    fn from_ptr(reader_ptr: *mut OTF2_Reader_struct) -> Self {
        OwnedExternHandle::new(reader_ptr, OTF2_Reader_Close)
    }

    fn set_serial_collective_callbacks(&mut self) -> Result<(), Status> {
        unsafe { OTF2_Reader_SetSerialCollectiveCallbacks(self.as_raw_mut()) }.into()
    }

    fn read_global_definitions(&mut self, visitors: &mut DefinitionVisitorMultiplexer) -> Result<(), Status> {
        let global_def_reader = unsafe { OTF2_Reader_GetGlobalDefReader(self.as_raw_mut()) };
        assert!(
            !global_def_reader.is_null(),
            "OTF2_Reader_GetGlobalDefReader returned null"
        );
        let mut global_callbacks = OwnedExternHandle::new(
            unsafe { OTF2_GlobalDefReaderCallbacks_New() },
            OTF2_GlobalDefReaderCallbacks_Delete,
        );
        visitors.set_global_def_reader_callbacks(&mut global_callbacks)?;
        unsafe {
            OTF2_Reader_RegisterGlobalDefCallbacks(
                self.as_raw_mut(),
                global_def_reader,
                global_callbacks.as_raw_mut(),
                visitors as *const _ as *mut _,
            )
        }?;
        let mut definitions_read: u64 = 0;
        unsafe {
            OTF2_Reader_ReadAllGlobalDefinitions(
                self.as_raw_mut(),
                global_def_reader,
                &mut definitions_read,
            )
        }?;
        unsafe { OTF2_Reader_CloseGlobalDefReader(self.as_raw_mut(), global_def_reader) }?;
        eprintln!("<{definitions_read} definitions were read>");
        // dbg!(&visitors);
        Ok(())
    }

    fn select_locations(&mut self, locations: impl Iterator<Item = OTF2_LocationRef>) -> Result<(), Status> {
        for location in locations {
            unsafe { OTF2_Reader_SelectLocation(self.as_raw_mut(), location) }?;
        }
        Ok(())
    }

    fn select_evt_readers(&mut self, locations: impl Iterator<Item = OTF2_LocationRef>) -> impl Iterator<Item = (OTF2_LocationRef, *mut OTF2_EvtReader)> {
        let handle_mut = unsafe { self.as_raw_mut() };
        locations.map(move |location| {
            let evt_reader = unsafe { OTF2_Reader_GetEvtReader(handle_mut, location) };
            assert!(!evt_reader.is_null(), "evt_reader pointer must be non-null");
            (location, evt_reader)
        })
    }
}

fn create_global_evt_reader_callbacks() -> GlobalEvtReaderCallbacks {
    let callbacks = unsafe { OTF2_GlobalEvtReaderCallbacks_New() };
    GlobalEvtReaderCallbacks::new(callbacks, OTF2_GlobalEvtReaderCallbacks_Delete)
}

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
    locations: LocationRegistry,
    strings: StringRegistry,
    attributes: AttributeRegistry,
}

impl Reader {
    pub fn open(anchor_file: String) -> Result<Self, ReaderError> {
        let anchor_file: CString = CString::new(anchor_file)
            .map_err(|e| ReaderError::NullByte("anchor_file", e.nul_position()))?;
        let reader_ptr = unsafe { OTF2_Reader_Open(anchor_file.as_ptr()) };
        if reader_ptr.is_null() {
            return Err(ReaderError::OpenReturnedNull(
                anchor_file.to_string_lossy().into_owned(),
            ));
        }
        let mut handle = ReaderHandle::from_ptr(reader_ptr);
        handle.set_serial_collective_callbacks().map_err(|_| ReaderError::Unknown)?;
        let mut locations = LocationRegistry::new();
        let mut strings = StringRegistry::new();
        let mut attributes = AttributeRegistry::new();
        let mut def_visitor = DefinitionVisitorMultiplexer::new();
        let mut printing_visitor = PrintingDefinitionVisitor::new();
        let mut strings = StringRegistry::new();
        let mut attributes = AttributeRegistry::new();
        // def_visitor.add_visitor(&mut printing_visitor);
        def_visitor.add_visitor(&mut strings);
        def_visitor.add_visitor(&mut attributes);
        def_visitor.add_visitor(&mut locations);
        handle.read_global_definitions(&mut def_visitor).map_err(|_| ReaderError::Unknown)?;
        handle.select_locations(locations.keys().cloned()).map_err(|_| ReaderError::Unknown)?;
        Ok(Self { handle, anchor_file, locations, strings, attributes})
    }

    pub fn read_all_events(&mut self) -> Result<(), Status> {
        let locations: Vec<OTF2_LocationRef> = self.locations.keys().cloned().collect();
        eprintln!(">> getting global evt reader");
        let global_evt_reader = self.get_global_evt_reader(&locations)?;
        let mut visitors = EventVisitorMultiplexer::new();
        let mut event_printer = PrintingEventVisitor::new();
        visitors.add_visitor(&mut event_printer);
        eprintln!(">> reading events");
        Reader::read_events(global_evt_reader, &mut visitors, 1000)?;
        eprintln!(">> events read");
        Ok(())
    }

    // fn set_serial_collective_callbacks(&mut self) -> Result<(), Status> {
    //     unsafe { OTF2_Reader_SetSerialCollectiveCallbacks(self.handle.as_raw_mut()) }.into()
    // }

    // fn read_global_definitions(&mut self, visitors: &mut DefinitionVisitorMultiplexer) -> Result<(), Status> {
    //     let global_def_reader = unsafe { OTF2_Reader_GetGlobalDefReader(self.handle.as_raw_mut()) };
    //     assert!(
    //         !global_def_reader.is_null(),
    //         "OTF2_Reader_GetGlobalDefReader returned null"
    //     );
    //     let mut global_callbacks = OwnedExternHandle::new(
    //         unsafe { OTF2_GlobalDefReaderCallbacks_New() },
    //         OTF2_GlobalDefReaderCallbacks_Delete,
    //     );
    //     visitors.set_global_def_reader_callbacks(&mut global_callbacks)?;
    //     unsafe {
    //         OTF2_Reader_RegisterGlobalDefCallbacks(
    //             self.handle.as_raw_mut(),
    //             global_def_reader,
    //             global_callbacks.as_raw_mut(),
    //             visitors as *const _ as *mut _,
    //         )
    //     }?;
    //     let mut definitions_read: u64 = 0;
    //     unsafe {
    //         OTF2_Reader_ReadAllGlobalDefinitions(
    //             self.handle.as_raw_mut(),
    //             global_def_reader,
    //             &mut definitions_read,
    //         )
    //     }?;
    //     unsafe { OTF2_Reader_CloseGlobalDefReader(self.handle.as_raw_mut(), global_def_reader) }?;
    //     eprintln!("<{definitions_read} definitions were read>");
    //     dbg!(&visitors);
    //     Ok(())
    // }

    fn read_events(global_evt_reader: *mut OTF2_GlobalEvtReader_struct, visitors: &mut EventVisitorMultiplexer, batch_size: u64) -> Result<(), Status> {
        let mut callbacks = create_global_evt_reader_callbacks();
        eprintln!(">> set global event reader callbacks");
        visitors.set_global_evt_reader_callbacks(&mut callbacks)?;
        eprintln!(">> register global event visitor callbacks");
        dbg!(&global_evt_reader);
        dbg!(&callbacks);
        let result: Result<_, _> = unsafe { OTF2_GlobalEvtReader_SetCallbacks(global_evt_reader, callbacks.as_raw(), visitors as *const _ as *mut _) }.into();
        dbg!(&result);
        assert_eq!(result, Ok(()));
        eprintln!(">> begin event loop");
        loop {
            let mut events_read = 0;
            unsafe { OTF2_GlobalEvtReader_ReadEvents(global_evt_reader, batch_size, &mut events_read) };
            if events_read < batch_size {
                break;
            }
        }
        eprintln!(">> end event loop");
        Ok(())
    }

    // fn select_locations(&mut self, locations: &[OTF2_LocationRef]) -> Result<(), Status> {
    //     for location in locations {
    //         self.select_location(*location)?;
    //     }
    //     Ok(())
    // }

    // fn select_location(&mut self, location: OTF2_LocationRef) -> Result<(), Status> {
    //     unsafe { OTF2_Reader_SelectLocation(self.handle.as_raw_mut(), location) }.into()
    // }

    fn get_global_evt_reader(&mut self, locations: &[OTF2_LocationRef]) -> Result<*mut OTF2_GlobalEvtReader_struct, Status> {
        self.read_def_files(locations)?;
        self.open_evt_files(locations)?;
        let evt_readers: Vec<_> = self.handle.select_evt_readers(locations.iter().cloned()).collect();
        dbg!(&evt_readers);
        dbg!(unsafe{self.handle.as_raw_mut()});
        let global_evt_reader = unsafe { OTF2_Reader_GetGlobalEvtReader(self.handle.as_raw_mut()) };
        dbg!(&global_evt_reader);
        assert!(!global_evt_reader.is_null(), "global_evt_reader pointer must be non-null");
        Ok(global_evt_reader)
    }

    fn read_def_files(&mut self, locations: &[OTF2_LocationRef]) -> Result<(), Status> {
        let result: Result<_, _> = unsafe { OTF2_Reader_OpenDefFiles(self.handle.as_raw_mut()).into() };
        for location in locations {
            self.read_local_definitions(*location)?;
        }
        unsafe { OTF2_Reader_CloseDefFiles(self.handle.as_raw_mut()) }?;
        result
    }

    fn read_local_definitions(&mut self, location: OTF2_LocationRef) -> Result<(), Status> {
        let def_reader = unsafe { OTF2_Reader_GetDefReader(self.handle.as_raw_mut(), location) };
        let mut definitions_read = 0;
        let result = unsafe { OTF2_Reader_ReadAllLocalDefinitions(self.handle.as_raw_mut(), def_reader, &mut definitions_read) }.into();
        unsafe { OTF2_Reader_CloseDefReader(self.handle.as_raw_mut(), def_reader) }?;
        result
    }

    fn open_evt_files(&mut self, locations: &[OTF2_LocationRef]) -> Result<(), Status> {
        unsafe { OTF2_Reader_OpenEvtFiles(self.handle.as_raw_mut()) }?;
        // for location in locations {
        //     // NOTE: ignore return value for now, but should manage this properly
        //     unsafe { OTF2_Reader_GetEvtReader(self.handle.as_raw_mut(), *location) };
        // }
        Ok(())
    }

    fn close_evt_files(&mut self) -> Result<(), Status> {
        unsafe { OTF2_Reader_CloseEvtFiles(self.handle.as_raw_mut()) }.into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_reader() {
        let mut reader = Reader::open("/home/adam/Dropbox/Durham-RA/experiments/bots-strassen/trace/serial_512.15132/serial_512.15132.otf2".to_string());
        // dbg!(&reader);
        let result = reader.unwrap().read_all_events();
        dbg!(&result);
    }
}
