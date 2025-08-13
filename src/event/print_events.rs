#![allow(unused_variables)]

use crate::error::Status;
use crate::internal::*;
use std::ffi::CStr;

use crate::event::EventVisitor;

// A visitor that prints all event information to stderr
#[derive(Debug)]
pub struct PrintingEventVisitor;

impl PrintingEventVisitor {
    pub fn new() -> Self {
        Self
    }
}

impl EventVisitor for PrintingEventVisitor {
    fn visit_thread_task_create_event(
        &mut self,
        location_id: OTF2_LocationRef,
        time: OTF2_TimeStamp,
        attribute_list: *mut OTF2_AttributeList,
        thread_team: OTF2_CommRef,
        creating_thread: u32,
        generation_number: u32,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "Thread Task Create Event at {} on location {}: thread_team {}, creating_thread {}, generation_number {}",
            time, location_id, thread_team, creating_thread, generation_number
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_thread_task_switch_event(
        &mut self,
        location_id: OTF2_LocationRef,
        time: OTF2_TimeStamp,
        attribute_list: *mut OTF2_AttributeList,
        thread_team: OTF2_CommRef,
        creating_thread: u32,
        generation_number: u32,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "Thread Task Switch Event at {} on location {}: thread_team {}, creating_thread {}, generation_number {}",
            time, location_id, thread_team, creating_thread, generation_number
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }
}
