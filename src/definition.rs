use crate::internal::*;
use crate::error::Status;
use std::ffi::CStr;

pub trait DefinitionVisitor {
    fn visit_string(&mut self, defn: OTF2_StringRef, value: &CStr) -> OTF2_CallbackCode;
    fn visit_location(&mut self, defn: OTF2_LocationRef, name: OTF2_StringRef, location_type: OTF2_LocationType, num_events: u64, location_group: OTF2_LocationGroupRef) -> OTF2_CallbackCode;
}

pub struct DefinitionVisitorMultiplexer {
    visitors: Vec<Box<dyn DefinitionVisitor>>,
}

impl DefinitionVisitorMultiplexer {
    pub fn new() -> Self {
        Self {
            visitors: Vec::new(),
        }
    }

    pub fn add_visitor(&mut self, visitor: Box<dyn DefinitionVisitor>) {
        self.visitors.push(visitor);
    }

    pub fn set_global_def_reader_callbacks<U>(&self, global_callbacks: &mut OwnedExternHandle<OTF2_GlobalDefReaderCallbacks_struct, U>) -> Result<(), Status> {
        unsafe {
            let cbs = global_callbacks.as_raw_mut();
            OTF2_GlobalDefReaderCallbacks_SetStringCallback(cbs, Some(read_string_def))?;
            OTF2_GlobalDefReaderCallbacks_SetLocationCallback(cbs, Some(read_location_def))?;
        }
        Ok(())
    }
}

extern "C" fn read_string_def(user_data: *mut ::std::os::raw::c_void, defn: OTF2_StringRef, value: *const ::std::os::raw::c_char) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    let value = unsafe { std::ffi::CStr::from_ptr(value) };
    eprintln!("visit_string: defn: {}, value: {}", defn, value.to_string_lossy());
    for visitor in &mut this.visitors {
        let code = visitor.visit_string(defn, value);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_location_def(user_data: *mut ::std::os::raw::c_void, defn: OTF2_LocationRef, name: OTF2_StringRef, location_type: OTF2_LocationType, num_events: u64, location_group: OTF2_LocationGroupRef) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!("visit_location: defn: {}, name: {}, location_type: {}, num_events: {}, location_group: {}", defn, name, location_type, num_events, location_group);
    for visitor in &mut this.visitors {
        let code = visitor.visit_location(defn, name, location_type, num_events, location_group);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}
