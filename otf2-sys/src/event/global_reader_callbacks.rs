#![allow(unused_variables)]

use crate::error::Status;
use crate::internal::*;
use crate::attribute::AttributeIterator;
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
                OTF2_GlobalEvtReaderCallbacks_SetBufferFlushCallback(
                    self.as_mut_ptr(),
                    Some(on_buffer_flush_event),
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
        let attributes: Vec<_> = Handle::from_raw_unchecked(attribute_list).into_iter().collect();
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_thread_task_create_event(location_id, time, &attributes, thread_team, creating_thread, generation_number);
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
        let attributes: Vec<_> = Handle::from_raw_unchecked(attribute_list).into_iter().collect();
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_thread_task_switch_event(location_id, time, &attributes, thread_team, creating_thread, generation_number);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_buffer_flush_event(
        location_id: OTF2_LocationRef,
        time: OTF2_TimeStamp,
        user_data: *mut ::std::os::raw::c_void,
        attribute_list: *mut OTF2_AttributeList,
        stop_time: u64,
    ) -> OTF2_CallbackCode {
        let attributes: Vec<_> = Handle::from_raw_unchecked(attribute_list).into_iter().collect();
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_buffer_flush_event(location_id, time, &attributes, stop_time);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_Unknown_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_BufferFlush_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, stopTime: OTF2_TimeStamp ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_MeasurementOnOff_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, measurementMode: OTF2_MeasurementMode ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_Enter_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, region: OTF2_RegionRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_Leave_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, region: OTF2_RegionRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_MpiSend_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, receiver: u32, communicator: OTF2_CommRef, msgTag: u32, msgLength: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_MpiIsend_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, receiver: u32, communicator: OTF2_CommRef, msgTag: u32, msgLength: u64, requestID: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_MpiIsendComplete_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, requestID: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_MpiIrecvRequest_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, requestID: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_MpiRecv_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, sender: u32, communicator: OTF2_CommRef, msgTag: u32, msgLength: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_MpiIrecv_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, sender: u32, communicator: OTF2_CommRef, msgTag: u32, msgLength: u64, requestID: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_MpiRequestTest_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, requestID: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_MpiRequestCancelled_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, requestID: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_MpiCollectiveBegin_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_MpiCollectiveEnd_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, collectiveOp: OTF2_CollectiveOp, communicator: OTF2_CommRef, root: u32, sizeSent: u64, sizeReceived: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_OmpFork_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, numberOfRequestedThreads: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_OmpJoin_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_OmpAcquireLock_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, lockID: u32, acquisitionOrder: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_OmpReleaseLock_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, lockID: u32, acquisitionOrder: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_OmpTaskCreate_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, taskID: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_OmpTaskSwitch_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, taskID: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_OmpTaskComplete_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, taskID: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_Metric_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, metric: OTF2_MetricRef, numberOfMetrics: u8, typeIDs: *const OTF2_Type, metricValues: *const OTF2_MetricValue ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_ParameterString_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, parameter: OTF2_ParameterRef, string: OTF2_StringRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_ParameterInt_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, parameter: OTF2_ParameterRef, value: i64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_ParameterUnsignedInt_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, parameter: OTF2_ParameterRef, value: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_RmaWinCreate_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, win: OTF2_RmaWinRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_RmaWinDestroy_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, win: OTF2_RmaWinRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_RmaCollectiveBegin_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_RmaCollectiveEnd_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, collectiveOp: OTF2_CollectiveOp, syncLevel: OTF2_RmaSyncLevel, win: OTF2_RmaWinRef, root: u32, bytesSent: u64, bytesReceived: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_RmaGroupSync_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, syncLevel: OTF2_RmaSyncLevel, win: OTF2_RmaWinRef, group: OTF2_GroupRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_RmaRequestLock_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, lockId: u64, lockType: OTF2_LockType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_RmaAcquireLock_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, lockId: u64, lockType: OTF2_LockType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_RmaTryLock_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, lockId: u64, lockType: OTF2_LockType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_RmaReleaseLock_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, lockId: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_RmaSync_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, syncType: OTF2_RmaSyncType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_RmaWaitChange_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, win: OTF2_RmaWinRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_RmaPut_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, bytes: u64, matchingId: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_RmaGet_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, bytes: u64, matchingId: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_RmaAtomic_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, type_: OTF2_RmaAtomicType, bytesSent: u64, bytesReceived: u64, matchingId: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_RmaOpCompleteBlocking_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, matchingId: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_RmaOpCompleteNonBlocking_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, matchingId: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_RmaOpTest_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, matchingId: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_RmaOpCompleteRemote_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, matchingId: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_ThreadFork_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, model: OTF2_Paradigm, numberOfRequestedThreads: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_ThreadJoin_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, model: OTF2_Paradigm ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_ThreadTeamBegin_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, threadTeam: OTF2_CommRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_ThreadTeamEnd_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, threadTeam: OTF2_CommRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_ThreadAcquireLock_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, model: OTF2_Paradigm, lockID: u32, acquisitionOrder: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_ThreadReleaseLock_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, model: OTF2_Paradigm, lockID: u32, acquisitionOrder: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_ThreadTaskCreate_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, threadTeam: OTF2_CommRef, creatingThread: u32, generationNumber: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_ThreadTaskSwitch_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, threadTeam: OTF2_CommRef, creatingThread: u32, generationNumber: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_ThreadTaskComplete_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, threadTeam: OTF2_CommRef, creatingThread: u32, generationNumber: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_ThreadCreate_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, threadContingent: OTF2_CommRef, sequenceCount: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_ThreadBegin_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, threadContingent: OTF2_CommRef, sequenceCount: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_ThreadWait_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, threadContingent: OTF2_CommRef, sequenceCount: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_ThreadEnd_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, threadContingent: OTF2_CommRef, sequenceCount: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_CallingContextEnter_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, callingContext: OTF2_CallingContextRef, unwindDistance: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_CallingContextLeave_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, callingContext: OTF2_CallingContextRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_CallingContextSample_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, callingContext: OTF2_CallingContextRef, unwindDistance: u32, interruptGenerator: OTF2_InterruptGeneratorRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_IoCreateHandle_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, mode: OTF2_IoAccessMode, creationFlags: OTF2_IoCreationFlag, statusFlags: OTF2_IoStatusFlag ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_IoDestroyHandle_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_IoDuplicateHandle_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, oldHandle: OTF2_IoHandleRef, newHandle: OTF2_IoHandleRef, statusFlags: OTF2_IoStatusFlag ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_IoSeek_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, offsetRequest: i64, whence: OTF2_IoSeekOption, offsetResult: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_IoChangeStatusFlags_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, statusFlags: OTF2_IoStatusFlag ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_IoDeleteFile_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, ioParadigm: OTF2_IoParadigmRef, file: OTF2_IoFileRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_IoOperationBegin_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, mode: OTF2_IoOperationMode, operationFlags: OTF2_IoOperationFlag, bytesRequest: u64, matchingId: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_IoOperationTest_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, matchingId: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_IoOperationIssued_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, matchingId: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_IoOperationComplete_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, bytesResult: u64, matchingId: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_IoOperationCancelled_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, matchingId: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_IoAcquireLock_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, lockType: OTF2_LockType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_IoReleaseLock_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, lockType: OTF2_LockType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_IoTryLock_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, lockType: OTF2_LockType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_ProgramBegin_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, programName: OTF2_StringRef, numberOfArguments: u32, programArguments: *const OTF2_StringRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_ProgramEnd_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, exitStatus: i64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_NonBlockingCollectiveRequest_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, requestID: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_NonBlockingCollectiveComplete_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, collectiveOp: OTF2_CollectiveOp, communicator: OTF2_CommRef, root: u32, sizeSent: u64, sizeReceived: u64, requestID: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_CommCreate_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, communicator: OTF2_CommRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_CommDestroy_event(locationID: OTF2_LocationRef, time: OTF2_TimeStamp, userData: *mut ::std::os::raw::c_void, attributeList: *mut OTF2_AttributeList, communicator: OTF2_CommRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

}
