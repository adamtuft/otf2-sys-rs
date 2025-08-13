#![allow(unused_variables)]

use crate::error::Status;
use crate::internal::*;
use std::ffi::CStr;

pub trait EventVisitor: std::fmt::Debug {
    fn visit_thread_task_create_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attribute_list: *mut OTF2_AttributeList, thread_team: OTF2_CommRef, creating_thread: u32, generation_number: u32) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_thread_task_switch_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attribute_list: *mut OTF2_AttributeList, thread_team: OTF2_CommRef, creating_thread: u32, generation_number: u32) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }
}

unsafe extern "C" fn on_thread_task_create_event(
    location_id: OTF2_LocationRef,
    time: OTF2_TimeStamp,
    user_data: *mut ::std::os::raw::c_void,
    attribute_list: *mut OTF2_AttributeList,
    thread_team: OTF2_CommRef,
    creating_thread: u32,
    generation_number: u32) -> OTF2_CallbackCode
{
    let this: &mut EventVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    for visitor in &mut this.visitors {
        let code = visitor.visit_thread_task_create_event(location_id, time, attribute_list, thread_team, creating_thread, generation_number);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

unsafe extern "C" fn on_thread_task_switch_event(
    location_id: OTF2_LocationRef,
    time: OTF2_TimeStamp,
    user_data: *mut ::std::os::raw::c_void,
    attribute_list: *mut OTF2_AttributeList,
    thread_team: OTF2_CommRef,
    creating_thread: u32,
    generation_number: u32) -> OTF2_CallbackCode
{
    let this: &mut EventVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    for visitor in &mut this.visitors {
        let code = visitor.visit_thread_task_switch_event(location_id, time, attribute_list, thread_team, creating_thread, generation_number);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

#[derive(Debug)]
pub struct EventVisitorMultiplexer<'a> {
    visitors: Vec<&'a mut dyn EventVisitor>,
}

impl<'a> EventVisitorMultiplexer<'a> {
    pub fn new() -> Self {
        Self {
            visitors: Vec::new(),
        }
    }

    pub fn add_visitor(&mut self, visitor: &'a mut (dyn EventVisitor + 'a)) {
        self.visitors.push(visitor);
    }

    pub fn set_global_evt_reader_callbacks<U>(
        &self,
        global_callbacks: &mut OwnedExternHandle<OTF2_GlobalEvtReaderCallbacks_struct, U>
    ) -> Result<(), Status> {
        unsafe {
            let cbs = global_callbacks.as_raw_mut();

            OTF2_GlobalEvtReaderCallbacks_SetThreadTaskCreateCallback(cbs, Some(on_thread_task_create_event));
            OTF2_GlobalEvtReaderCallbacks_SetThreadTaskSwitchCallback(cbs, Some(on_thread_task_switch_event));
        }
        Ok(())
    }
}
