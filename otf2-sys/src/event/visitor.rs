#![allow(unused_variables)]

use crate::error::Status;
use crate::internal::*;
use crate::attribute::AttributeValue;
use std::ffi::CStr;

pub trait EventVisitor: std::fmt::Debug {
    fn visit_thread_task_create_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], thread_team: OTF2_CommRef, creating_thread: u32, generation_number: u32) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_thread_task_switch_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], thread_team: OTF2_CommRef, creating_thread: u32, generation_number: u32) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }
}
