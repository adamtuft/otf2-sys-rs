#![allow(unused_variables)]

use crate::error::Status;
use crate::internal::*;
use std::ffi::CStr;

use super::visitor::EventVisitor;

/// Safe wrapper around OTF2_GlobalEvtReaderCallbacks
/// 
/// Registers callbacks for reading events in OTF2 traces. These callbacks expect a
/// vector of mutable references to `EventVisitor` trait objects, each of which will be
/// notified of the events as they are read from the trace.
#[derive(Debug, derive_more::Deref, derive_more::DerefMut)]
pub struct GlobalEvtReaderCallbacks(Handle<OTF2_GlobalEvtReaderCallbacks_struct>);

impl core::ops::Drop for GlobalEvtReaderCallbacks {
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { OTF2_GlobalEvtReaderCallbacks_Delete(self.take()) };
        }
    }
}

impl GlobalEvtReaderCallbacks {
    pub fn new() -> Status<Self> {
        let mut this = Self(Handle::from_raw(unsafe { OTF2_GlobalEvtReaderCallbacks_New() })
            .expect("Failed to create GlobalEvtReaderCallbacks: null pointer"));
        this.set_callbacks()?;
        Ok(this)
    }

    fn set_callbacks(&mut self) -> Status<()> {
        use visitor_callbacks::*;
        unsafe {
            OTF2_GlobalEvtReaderCallbacks_SetThreadTaskCreateCallback(
                self.as_mut_ptr(),
                Some(on_thread_task_create_event),
            )?;
            OTF2_GlobalEvtReaderCallbacks_SetThreadTaskSwitchCallback(
                self.as_mut_ptr(),
                Some(on_thread_task_switch_event),
            )?;
        }
        Ok(())
    }
}

mod visitor_callbacks {
    use super::*;

    #[inline]
    fn as_visitors<'a, 'b>(data: *mut ::std::os::raw::c_void) -> &'a mut Vec<&'b mut dyn EventVisitor> {
        unsafe { &mut *(data as *mut Vec<&mut dyn EventVisitor>) }
    }

    pub unsafe extern "C" fn on_thread_task_create_event(
        location_id: OTF2_LocationRef,
        time: OTF2_TimeStamp,
        user_data: *mut ::std::os::raw::c_void,
        attribute_list: *mut OTF2_AttributeList,
        thread_team: OTF2_CommRef,
        creating_thread: u32,
        generation_number: u32) -> OTF2_CallbackCode
    {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_thread_task_create_event(location_id, time, attribute_list, thread_team, creating_thread, generation_number);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_thread_task_switch_event(
        location_id: OTF2_LocationRef,
        time: OTF2_TimeStamp,
        user_data: *mut ::std::os::raw::c_void,
        attribute_list: *mut OTF2_AttributeList,
        thread_team: OTF2_CommRef,
        creating_thread: u32,
        generation_number: u32) -> OTF2_CallbackCode
    {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_thread_task_switch_event(location_id, time, attribute_list, thread_team, creating_thread, generation_number);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }
}