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

    pub unsafe extern "C" fn on_unknown_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_buffer_flush_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, stop_time: OTF2_TimeStamp ) -> OTF2_CallbackCode {
        let attributes: Vec<_> = Handle::from_raw_unchecked(attribute_list).into_iter().collect();
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_buffer_flush_event(location_id, time, &attributes, stop_time);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_measurement_on_off_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, measurement_mode: OTF2_MeasurementMode ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_enter_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, region: OTF2_RegionRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_leave_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, region: OTF2_RegionRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_mpi_send_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, receiver: u32, communicator: OTF2_CommRef, msg_tag: u32, msg_length: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_mpi_isend_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, receiver: u32, communicator: OTF2_CommRef, msg_tag: u32, msg_length: u64, request_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_mpi_isend_complete_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, request_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_mpi_irecv_request_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, request_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_mpi_recv_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, sender: u32, communicator: OTF2_CommRef, msg_tag: u32, msg_length: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_mpi_irecv_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, sender: u32, communicator: OTF2_CommRef, msg_tag: u32, msg_length: u64, request_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_mpi_request_test_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, request_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_mpi_request_cancelled_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, request_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_mpi_collective_begin_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_mpi_collective_end_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, collective_op: OTF2_CollectiveOp, communicator: OTF2_CommRef, root: u32, size_sent: u64, size_received: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_omp_fork_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, number_of_requested_threads: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_omp_join_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_omp_acquire_lock_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, lock_id: u32, acquisition_order: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_omp_release_lock_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, lock_id: u32, acquisition_order: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_omp_task_create_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, task_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_omp_task_switch_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, task_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_omp_task_complete_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, task_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_metric_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, metric: OTF2_MetricRef, number_of_metrics: u8, type_ids: *const OTF2_Type, metric_values: *const OTF2_MetricValue ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_parameter_string_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, parameter: OTF2_ParameterRef, string: OTF2_StringRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_parameter_int_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, parameter: OTF2_ParameterRef, value: i64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_parameter_unsigned_int_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, parameter: OTF2_ParameterRef, value: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_rma_win_create_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, win: OTF2_RmaWinRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_rma_win_destroy_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, win: OTF2_RmaWinRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_rma_collective_begin_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_rma_collective_end_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, collective_op: OTF2_CollectiveOp, sync_level: OTF2_RmaSyncLevel, win: OTF2_RmaWinRef, root: u32, bytes_sent: u64, bytes_received: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_rma_group_sync_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, sync_level: OTF2_RmaSyncLevel, win: OTF2_RmaWinRef, group: OTF2_GroupRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_rma_request_lock_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, lock_id: u64, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_rma_acquire_lock_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, lock_id: u64, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_rma_try_lock_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, lock_id: u64, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_rma_release_lock_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, lock_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_rma_sync_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, sync_type: OTF2_RmaSyncType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_rma_wait_change_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, win: OTF2_RmaWinRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_rma_put_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, bytes: u64, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_rma_get_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, bytes: u64, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_rma_atomic_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, type_: OTF2_RmaAtomicType, bytes_sent: u64, bytes_received: u64, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_rma_op_complete_blocking_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_rma_op_complete_non_blocking_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_rma_op_test_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_rma_op_complete_remote_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_thread_fork_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, model: OTF2_Paradigm, number_of_requested_threads: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_thread_join_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, model: OTF2_Paradigm ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_thread_team_begin_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, thread_team: OTF2_CommRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_thread_team_end_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, thread_team: OTF2_CommRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_thread_acquire_lock_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, model: OTF2_Paradigm, lock_id: u32, acquisition_order: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_thread_release_lock_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, model: OTF2_Paradigm, lock_id: u32, acquisition_order: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_thread_task_create_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, thread_team: OTF2_CommRef, creating_thread: u32, generation_number: u32 ) -> OTF2_CallbackCode {
        let attributes: Vec<_> = Handle::from_raw_unchecked(attribute_list).into_iter().collect();
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_thread_task_create_event(location_id, time, &attributes, thread_team, creating_thread, generation_number);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_thread_task_switch_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, thread_team: OTF2_CommRef, creating_thread: u32, generation_number: u32 ) -> OTF2_CallbackCode {
        let attributes: Vec<_> = Handle::from_raw_unchecked(attribute_list).into_iter().collect();
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_thread_task_switch_event(location_id, time, &attributes, thread_team, creating_thread, generation_number);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_thread_task_complete_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, thread_team: OTF2_CommRef, creating_thread: u32, generation_number: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_thread_create_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, thread_contingent: OTF2_CommRef, sequence_count: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_thread_begin_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, thread_contingent: OTF2_CommRef, sequence_count: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_thread_wait_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, thread_contingent: OTF2_CommRef, sequence_count: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_thread_end_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, thread_contingent: OTF2_CommRef, sequence_count: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_calling_context_enter_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, calling_context: OTF2_CallingContextRef, unwind_distance: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_calling_context_leave_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, calling_context: OTF2_CallingContextRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_calling_context_sample_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, calling_context: OTF2_CallingContextRef, unwind_distance: u32, interrupt_generator: OTF2_InterruptGeneratorRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_io_create_handle_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, mode: OTF2_IoAccessMode, creation_flags: OTF2_IoCreationFlag, status_flags: OTF2_IoStatusFlag ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_io_destroy_handle_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_io_duplicate_handle_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, old_handle: OTF2_IoHandleRef, new_handle: OTF2_IoHandleRef, status_flags: OTF2_IoStatusFlag ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_io_seek_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, offset_request: i64, whence: OTF2_IoSeekOption, offset_result: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_io_change_status_flags_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, status_flags: OTF2_IoStatusFlag ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_io_delete_file_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, io_paradigm: OTF2_IoParadigmRef, file: OTF2_IoFileRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_io_operation_begin_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, mode: OTF2_IoOperationMode, operation_flags: OTF2_IoOperationFlag, bytes_request: u64, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_io_operation_test_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_io_operation_issued_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_io_operation_complete_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, bytes_result: u64, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_io_operation_cancelled_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_io_acquire_lock_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_io_release_lock_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_io_try_lock_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_program_begin_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, program_name: OTF2_StringRef, number_of_arguments: u32, program_arguments: *const OTF2_StringRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_program_end_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, exit_status: i64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_non_blocking_collective_request_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, request_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_non_blocking_collective_complete_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, collective_op: OTF2_CollectiveOp, communicator: OTF2_CommRef, root: u32, size_sent: u64, size_received: u64, request_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_comm_create_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, communicator: OTF2_CommRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub unsafe extern "C" fn on_comm_destroy_event(location_id: OTF2_LocationRef, time: OTF2_TimeStamp, user_data: *mut ::std::os::raw::c_void, attribute_list: *mut OTF2_AttributeList, communicator: OTF2_CommRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }
}
