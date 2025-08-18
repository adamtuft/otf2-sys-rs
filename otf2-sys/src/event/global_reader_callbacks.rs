#![allow(unused_variables)]

use crate::error::Status;
use crate::internal::*;
use crate::attribute::AttributeIterator;
use std::ffi::CStr;

use super::visitor::{Event, EventKind};

use std::collections::VecDeque;

/// Safe wrapper around OTF2_GlobalEvtReaderCallbacks
/// 
/// Registers callbacks for reading events in OTF2 traces.
#[derive(Debug, derive_more::Deref, derive_more::DerefMut)]
pub struct GlobalEvtReaderCallbacks(Handle<OTF2_GlobalEvtReaderCallbacks_struct>);

impl core::ops::Drop for GlobalEvtReaderCallbacks {
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { OTF2_GlobalEvtReaderCallbacks_Delete(self.take()) };
        }
    }
}

macro_rules! set_callbacks {
    ($handle:ident, $($setter:ident => $callback:ident),* $(,)?) => {
        $( $setter(
            $handle.as_mut_ptr(),
            Some($callback),
        )?;)*
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
            set_callbacks!(self,
                OTF2_GlobalEvtReaderCallbacks_SetUnknownCallback => on_unknown_event,
                OTF2_GlobalEvtReaderCallbacks_SetBufferFlushCallback => on_buffer_flush_event,
                OTF2_GlobalEvtReaderCallbacks_SetMeasurementOnOffCallback => on_measurement_on_off_event,
                OTF2_GlobalEvtReaderCallbacks_SetEnterCallback => on_enter_event,
                OTF2_GlobalEvtReaderCallbacks_SetLeaveCallback => on_leave_event,
                OTF2_GlobalEvtReaderCallbacks_SetMpiSendCallback => on_mpi_send_event,
                OTF2_GlobalEvtReaderCallbacks_SetMpiIsendCallback => on_mpi_isend_event,
                OTF2_GlobalEvtReaderCallbacks_SetMpiIsendCompleteCallback => on_mpi_isend_complete_event,
                OTF2_GlobalEvtReaderCallbacks_SetMpiIrecvRequestCallback => on_mpi_irecv_request_event,
                OTF2_GlobalEvtReaderCallbacks_SetMpiRecvCallback => on_mpi_recv_event,
                OTF2_GlobalEvtReaderCallbacks_SetMpiIrecvCallback => on_mpi_irecv_event,
                OTF2_GlobalEvtReaderCallbacks_SetMpiRequestTestCallback => on_mpi_request_test_event,
                OTF2_GlobalEvtReaderCallbacks_SetMpiRequestCancelledCallback => on_mpi_request_cancelled_event,
                OTF2_GlobalEvtReaderCallbacks_SetMpiCollectiveBeginCallback => on_mpi_collective_begin_event,
                OTF2_GlobalEvtReaderCallbacks_SetMpiCollectiveEndCallback => on_mpi_collective_end_event,
                OTF2_GlobalEvtReaderCallbacks_SetOmpForkCallback => on_omp_fork_event,
                OTF2_GlobalEvtReaderCallbacks_SetOmpJoinCallback => on_omp_join_event,
                OTF2_GlobalEvtReaderCallbacks_SetOmpAcquireLockCallback => on_omp_acquire_lock_event,
                OTF2_GlobalEvtReaderCallbacks_SetOmpReleaseLockCallback => on_omp_release_lock_event,
                OTF2_GlobalEvtReaderCallbacks_SetOmpTaskCreateCallback => on_omp_task_create_event,
                OTF2_GlobalEvtReaderCallbacks_SetOmpTaskSwitchCallback => on_omp_task_switch_event,
                OTF2_GlobalEvtReaderCallbacks_SetOmpTaskCompleteCallback => on_omp_task_complete_event,
                OTF2_GlobalEvtReaderCallbacks_SetMetricCallback => on_metric_event,
                OTF2_GlobalEvtReaderCallbacks_SetParameterStringCallback => on_parameter_string_event,
                OTF2_GlobalEvtReaderCallbacks_SetParameterIntCallback => on_parameter_int_event,
                OTF2_GlobalEvtReaderCallbacks_SetParameterUnsignedIntCallback => on_parameter_unsigned_int_event,
                OTF2_GlobalEvtReaderCallbacks_SetRmaWinCreateCallback => on_rma_win_create_event,
                OTF2_GlobalEvtReaderCallbacks_SetRmaWinDestroyCallback => on_rma_win_destroy_event,
                OTF2_GlobalEvtReaderCallbacks_SetRmaCollectiveBeginCallback => on_rma_collective_begin_event,
                OTF2_GlobalEvtReaderCallbacks_SetRmaCollectiveEndCallback => on_rma_collective_end_event,
                OTF2_GlobalEvtReaderCallbacks_SetRmaGroupSyncCallback => on_rma_group_sync_event,
                OTF2_GlobalEvtReaderCallbacks_SetRmaRequestLockCallback => on_rma_request_lock_event,
                OTF2_GlobalEvtReaderCallbacks_SetRmaAcquireLockCallback => on_rma_acquire_lock_event,
                OTF2_GlobalEvtReaderCallbacks_SetRmaTryLockCallback => on_rma_try_lock_event,
                OTF2_GlobalEvtReaderCallbacks_SetRmaReleaseLockCallback => on_rma_release_lock_event,
                OTF2_GlobalEvtReaderCallbacks_SetRmaSyncCallback => on_rma_sync_event,
                OTF2_GlobalEvtReaderCallbacks_SetRmaWaitChangeCallback => on_rma_wait_change_event,
                OTF2_GlobalEvtReaderCallbacks_SetRmaPutCallback => on_rma_put_event,
                OTF2_GlobalEvtReaderCallbacks_SetRmaGetCallback => on_rma_get_event,
                OTF2_GlobalEvtReaderCallbacks_SetRmaAtomicCallback => on_rma_atomic_event,
                OTF2_GlobalEvtReaderCallbacks_SetRmaOpCompleteBlockingCallback => on_rma_op_complete_blocking_event,
                OTF2_GlobalEvtReaderCallbacks_SetRmaOpCompleteNonBlockingCallback => on_rma_op_complete_non_blocking_event,
                OTF2_GlobalEvtReaderCallbacks_SetRmaOpTestCallback => on_rma_op_test_event,
                OTF2_GlobalEvtReaderCallbacks_SetRmaOpCompleteRemoteCallback => on_rma_op_complete_remote_event,
                OTF2_GlobalEvtReaderCallbacks_SetThreadForkCallback => on_thread_fork_event,
                OTF2_GlobalEvtReaderCallbacks_SetThreadJoinCallback => on_thread_join_event,
                OTF2_GlobalEvtReaderCallbacks_SetThreadTeamBeginCallback => on_thread_team_begin_event,
                OTF2_GlobalEvtReaderCallbacks_SetThreadTeamEndCallback => on_thread_team_end_event,
                OTF2_GlobalEvtReaderCallbacks_SetThreadAcquireLockCallback => on_thread_acquire_lock_event,
                OTF2_GlobalEvtReaderCallbacks_SetThreadReleaseLockCallback => on_thread_release_lock_event,
                OTF2_GlobalEvtReaderCallbacks_SetThreadTaskCreateCallback => on_thread_task_create_event,
                OTF2_GlobalEvtReaderCallbacks_SetThreadTaskSwitchCallback => on_thread_task_switch_event,
                OTF2_GlobalEvtReaderCallbacks_SetThreadTaskCompleteCallback => on_thread_task_complete_event,
                OTF2_GlobalEvtReaderCallbacks_SetThreadCreateCallback => on_thread_create_event,
                OTF2_GlobalEvtReaderCallbacks_SetThreadBeginCallback => on_thread_begin_event,
                OTF2_GlobalEvtReaderCallbacks_SetThreadWaitCallback => on_thread_wait_event,
                OTF2_GlobalEvtReaderCallbacks_SetThreadEndCallback => on_thread_end_event,
                OTF2_GlobalEvtReaderCallbacks_SetCallingContextEnterCallback => on_calling_context_enter_event,
                OTF2_GlobalEvtReaderCallbacks_SetCallingContextLeaveCallback => on_calling_context_leave_event,
                OTF2_GlobalEvtReaderCallbacks_SetCallingContextSampleCallback => on_calling_context_sample_event,
                OTF2_GlobalEvtReaderCallbacks_SetIoCreateHandleCallback => on_io_create_handle_event,
                OTF2_GlobalEvtReaderCallbacks_SetIoDestroyHandleCallback => on_io_destroy_handle_event,
                OTF2_GlobalEvtReaderCallbacks_SetIoDuplicateHandleCallback => on_io_duplicate_handle_event,
                OTF2_GlobalEvtReaderCallbacks_SetIoSeekCallback => on_io_seek_event,
                OTF2_GlobalEvtReaderCallbacks_SetIoChangeStatusFlagsCallback => on_io_change_status_flags_event,
                OTF2_GlobalEvtReaderCallbacks_SetIoDeleteFileCallback => on_io_delete_file_event,
                OTF2_GlobalEvtReaderCallbacks_SetIoOperationBeginCallback => on_io_operation_begin_event,
                OTF2_GlobalEvtReaderCallbacks_SetIoOperationTestCallback => on_io_operation_test_event,
                OTF2_GlobalEvtReaderCallbacks_SetIoOperationIssuedCallback => on_io_operation_issued_event,
                OTF2_GlobalEvtReaderCallbacks_SetIoOperationCompleteCallback => on_io_operation_complete_event,
                OTF2_GlobalEvtReaderCallbacks_SetIoOperationCancelledCallback => on_io_operation_cancelled_event,
                OTF2_GlobalEvtReaderCallbacks_SetIoAcquireLockCallback => on_io_acquire_lock_event,
                OTF2_GlobalEvtReaderCallbacks_SetIoReleaseLockCallback => on_io_release_lock_event,
                OTF2_GlobalEvtReaderCallbacks_SetIoTryLockCallback => on_io_try_lock_event,
                OTF2_GlobalEvtReaderCallbacks_SetProgramBeginCallback => on_program_begin_event,
                OTF2_GlobalEvtReaderCallbacks_SetProgramEndCallback => on_program_end_event,
                OTF2_GlobalEvtReaderCallbacks_SetNonBlockingCollectiveRequestCallback => on_non_blocking_collective_request_event,
                OTF2_GlobalEvtReaderCallbacks_SetNonBlockingCollectiveCompleteCallback => on_non_blocking_collective_complete_event,
                OTF2_GlobalEvtReaderCallbacks_SetCommCreateCallback => on_comm_create_event,
                OTF2_GlobalEvtReaderCallbacks_SetCommDestroyCallback => on_comm_destroy_event,
            );
        }
        Ok(())
    }
}

mod visitor_callbacks {
    use super::*;
    use std::os::raw::c_void;

    #[inline]
    fn as_event_queue<'a>(data: *mut c_void) -> &'a mut VecDeque<Event> {
        assert!(!data.is_null(), "callback user data must not be null pointer");
        unsafe { &mut *(data as *mut _) }
    }

    macro_rules! into_attributes {
        ($list:ident) => {
            Handle::from_raw_unchecked($list).into_iter().collect::<Vec<_>>()
        }
    }

    macro_rules! push_event {
        ($queue:ident, $location:ident, $time:ident, $attr:ident, $kind:expr) => {{
            as_event_queue($queue).push_back(Event::new($location, $time, into_attributes!($attr), $kind));
            OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
        }}
    }

    pub unsafe extern "C" fn on_unknown_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::Unknown)
    }

    pub unsafe extern "C" fn on_buffer_flush_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, stop_time: OTF2_TimeStamp ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::BufferFlush { stop_time })
    }

    pub unsafe extern "C" fn on_measurement_on_off_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, measurement_mode: OTF2_MeasurementMode ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MeasurementOnOff { measurement_mode })
    }

    pub unsafe extern "C" fn on_enter_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, region: OTF2_RegionRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::Enter { region })
    }

    pub unsafe extern "C" fn on_leave_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, region: OTF2_RegionRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::Leave { region })
    }

    pub unsafe extern "C" fn on_mpi_send_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, receiver: u32, communicator: OTF2_CommRef, msg_tag: u32, msg_length: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MpiSend { receiver, communicator, msg_tag, msg_length })
    }

    pub unsafe extern "C" fn on_mpi_isend_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, receiver: u32, communicator: OTF2_CommRef, msg_tag: u32, msg_length: u64, request_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MpiIsend { receiver, communicator, msg_tag, msg_length, request_id })
    }

    pub unsafe extern "C" fn on_mpi_isend_complete_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, request_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MpiIsendComplete { request_id })
    }

    pub unsafe extern "C" fn on_mpi_irecv_request_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, request_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MpiIrecvRequest { request_id })
    }

    pub unsafe extern "C" fn on_mpi_recv_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, sender: u32, communicator: OTF2_CommRef, msg_tag: u32, msg_length: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MpiRecv { sender, communicator, msg_tag, msg_length })
    }

    pub unsafe extern "C" fn on_mpi_irecv_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, sender: u32, communicator: OTF2_CommRef, msg_tag: u32, msg_length: u64, request_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MpiIrecv { sender, communicator, msg_tag, msg_length, request_id })
    }

    pub unsafe extern "C" fn on_mpi_request_test_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, request_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MpiRequestTest { request_id })
    }

    pub unsafe extern "C" fn on_mpi_request_cancelled_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, request_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MpiRequestCancelled { request_id })
    }

    pub unsafe extern "C" fn on_mpi_collective_begin_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MpiCollectiveBegin)
    }

    pub unsafe extern "C" fn on_mpi_collective_end_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, collective_op: OTF2_CollectiveOp, communicator: OTF2_CommRef, root: u32, size_sent: u64, size_received: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MpiCollectiveEnd { collective_op, communicator, root, size_sent, size_received })
    }

    pub unsafe extern "C" fn on_omp_fork_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, number_of_requested_threads: u32 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::OmpFork { number_of_requested_threads })
    }

    pub unsafe extern "C" fn on_omp_join_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::OmpJoin)
    }

    pub unsafe extern "C" fn on_omp_acquire_lock_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, lock_id: u32, acquisition_order: u32 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::OmpAcquireLock { lock_id, acquisition_order })
    }

    pub unsafe extern "C" fn on_omp_release_lock_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, lock_id: u32, acquisition_order: u32 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::OmpReleaseLock { lock_id, acquisition_order })
    }

    pub unsafe extern "C" fn on_omp_task_create_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, task_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::OmpTaskCreate { task_id })
    }

    pub unsafe extern "C" fn on_omp_task_switch_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, task_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::OmpTaskSwitch { task_id })
    }

    pub unsafe extern "C" fn on_omp_task_complete_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, task_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::OmpTaskComplete { task_id })
    }

    pub unsafe extern "C" fn on_metric_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, metric: OTF2_MetricRef, number_of_metrics: u8, type_ids: *const OTF2_Type, metric_values: *const OTF2_MetricValue ) -> OTF2_CallbackCode {
        let type_ids = unsafe { std::slice::from_raw_parts(type_ids, number_of_metrics as usize) }.to_vec();
        let metric_values = unsafe { std::slice::from_raw_parts(metric_values, number_of_metrics as usize) }.to_vec();
        push_event!(queue, location, time, attributes, EventKind::Metric { metric, type_ids, metric_values })
    }

    pub unsafe extern "C" fn on_parameter_string_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, parameter: OTF2_ParameterRef, string: OTF2_StringRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ParameterString { parameter, string })
    }

    pub unsafe extern "C" fn on_parameter_int_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, parameter: OTF2_ParameterRef, value: i64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ParameterInt { parameter, value })
    }

    pub unsafe extern "C" fn on_parameter_unsigned_int_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, parameter: OTF2_ParameterRef, value: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ParameterUnsignedInt { parameter, value })
    }

    pub unsafe extern "C" fn on_rma_win_create_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaWinCreate { win })
    }

    pub unsafe extern "C" fn on_rma_win_destroy_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaWinDestroy { win })
    }

    pub unsafe extern "C" fn on_rma_collective_begin_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaCollectiveBegin)
    }

    pub unsafe extern "C" fn on_rma_collective_end_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, collective_op: OTF2_CollectiveOp, sync_level: OTF2_RmaSyncLevel, win: OTF2_RmaWinRef, root: u32, bytes_sent: u64, bytes_received: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaCollectiveEnd { collective_op, sync_level, win, root, bytes_sent, bytes_received })
    }

    pub unsafe extern "C" fn on_rma_group_sync_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, sync_level: OTF2_RmaSyncLevel, win: OTF2_RmaWinRef, group: OTF2_GroupRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaGroupSync { sync_level, win, group })
    }

    pub unsafe extern "C" fn on_rma_request_lock_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, lock_id: u64, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaRequestLock { win, remote, lock_id, lock_type })
    }

    pub unsafe extern "C" fn on_rma_acquire_lock_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, lock_id: u64, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaAcquireLock { win, remote, lock_id, lock_type })
    }

    pub unsafe extern "C" fn on_rma_try_lock_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, lock_id: u64, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaTryLock { win, remote, lock_id, lock_type })
    }

    pub unsafe extern "C" fn on_rma_release_lock_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, lock_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaReleaseLock { win, remote, lock_id })
    }

    pub unsafe extern "C" fn on_rma_sync_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, sync_type: OTF2_RmaSyncType ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaSync { win, remote, sync_type })
    }

    pub unsafe extern "C" fn on_rma_wait_change_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaWaitChange { win })
    }

    pub unsafe extern "C" fn on_rma_put_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, bytes: u64, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaPut { win, remote, bytes, matching_id })
    }

    pub unsafe extern "C" fn on_rma_get_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, bytes: u64, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaGet { win, remote, bytes, matching_id })
    }

    pub unsafe extern "C" fn on_rma_atomic_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, type_: OTF2_RmaAtomicType, bytes_sent: u64, bytes_received: u64, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaAtomic { win, remote, type_, bytes_sent, bytes_received, matching_id })
    }

    pub unsafe extern "C" fn on_rma_op_complete_blocking_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaOpCompleteBlocking { win, matching_id })
    }

    pub unsafe extern "C" fn on_rma_op_complete_non_blocking_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaOpCompleteNonBlocking { win, matching_id })
    }

    pub unsafe extern "C" fn on_rma_op_test_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaOpTest { win, matching_id })
    }

    pub unsafe extern "C" fn on_rma_op_complete_remote_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaOpCompleteRemote { win, matching_id })
    }

    pub unsafe extern "C" fn on_thread_fork_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, model: OTF2_Paradigm, number_of_requested_threads: u32 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadFork { model, number_of_requested_threads })
    }

    pub unsafe extern "C" fn on_thread_join_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, model: OTF2_Paradigm ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadJoin { model })
    }

    pub unsafe extern "C" fn on_thread_team_begin_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, thread_team: OTF2_CommRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadTeamBegin { thread_team })
    }

    pub unsafe extern "C" fn on_thread_team_end_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, thread_team: OTF2_CommRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadTeamEnd { thread_team })
    }

    pub unsafe extern "C" fn on_thread_acquire_lock_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, model: OTF2_Paradigm, lock_id: u32, acquisition_order: u32 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadAcquireLock { model, lock_id, acquisition_order })
    }

    pub unsafe extern "C" fn on_thread_release_lock_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, model: OTF2_Paradigm, lock_id: u32, acquisition_order: u32 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadReleaseLock { model, lock_id, acquisition_order })
    }

    pub unsafe extern "C" fn on_thread_task_create_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, thread_team: OTF2_CommRef, creating_thread: u32, generation_number: u32 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadTaskCreate { thread_team, creating_thread, generation_number })
    }

    pub unsafe extern "C" fn on_thread_task_switch_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, thread_team: OTF2_CommRef, creating_thread: u32, generation_number: u32 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadTaskSwitch { thread_team, creating_thread, generation_number })
    }

    pub unsafe extern "C" fn on_thread_task_complete_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, thread_team: OTF2_CommRef, creating_thread: u32, generation_number: u32 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadTaskComplete { thread_team, creating_thread, generation_number })
    }

    pub unsafe extern "C" fn on_thread_create_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, thread_contingent: OTF2_CommRef, sequence_count: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadCreate { thread_contingent, sequence_count })
    }

    pub unsafe extern "C" fn on_thread_begin_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, thread_contingent: OTF2_CommRef, sequence_count: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadBegin { thread_contingent, sequence_count })
    }

    pub unsafe extern "C" fn on_thread_wait_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, thread_contingent: OTF2_CommRef, sequence_count: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadWait { thread_contingent, sequence_count })
    }

    pub unsafe extern "C" fn on_thread_end_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, thread_contingent: OTF2_CommRef, sequence_count: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadEnd { thread_contingent, sequence_count })
    }

    pub unsafe extern "C" fn on_calling_context_enter_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, calling_context: OTF2_CallingContextRef, unwind_distance: u32 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::CallingContextEnter { calling_context, unwind_distance })
    }

    pub unsafe extern "C" fn on_calling_context_leave_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, calling_context: OTF2_CallingContextRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::CallingContextLeave { calling_context })
    }

    pub unsafe extern "C" fn on_calling_context_sample_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, calling_context: OTF2_CallingContextRef, unwind_distance: u32, interrupt_generator: OTF2_InterruptGeneratorRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::CallingContextSample { calling_context, unwind_distance, interrupt_generator })
    }

    pub unsafe extern "C" fn on_io_create_handle_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, mode: OTF2_IoAccessMode, creation_flags: OTF2_IoCreationFlag, status_flags: OTF2_IoStatusFlag ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoCreateHandle { handle, mode, creation_flags, status_flags })
    }

    pub unsafe extern "C" fn on_io_destroy_handle_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoDestroyHandle { handle })
    }

    pub unsafe extern "C" fn on_io_duplicate_handle_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, old_handle: OTF2_IoHandleRef, new_handle: OTF2_IoHandleRef, status_flags: OTF2_IoStatusFlag ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoDuplicateHandle { old_handle, new_handle, status_flags })
    }

    pub unsafe extern "C" fn on_io_seek_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, offset_request: i64, whence: OTF2_IoSeekOption, offset_result: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoSeek { handle, offset_request, whence, offset_result })
    }

    pub unsafe extern "C" fn on_io_change_status_flags_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, status_flags: OTF2_IoStatusFlag ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoChangeStatusFlags { handle, status_flags })
    }

    pub unsafe extern "C" fn on_io_delete_file_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, io_paradigm: OTF2_IoParadigmRef, file: OTF2_IoFileRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoDeleteFile { io_paradigm, file })
    }

    pub unsafe extern "C" fn on_io_operation_begin_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, mode: OTF2_IoOperationMode, operation_flags: OTF2_IoOperationFlag, bytes_request: u64, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoOperationBegin { handle, mode, operation_flags, bytes_request, matching_id })
    }

    pub unsafe extern "C" fn on_io_operation_test_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoOperationTest { handle, matching_id })
    }

    pub unsafe extern "C" fn on_io_operation_issued_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoOperationIssued { handle, matching_id })
    }

    pub unsafe extern "C" fn on_io_operation_complete_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, bytes_result: u64, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoOperationComplete { handle, bytes_result, matching_id })
    }

    pub unsafe extern "C" fn on_io_operation_cancelled_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoOperationCancelled { handle, matching_id })
    }

    pub unsafe extern "C" fn on_io_acquire_lock_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoAcquireLock { handle, lock_type })
    }

    pub unsafe extern "C" fn on_io_release_lock_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoReleaseLock { handle, lock_type })
    }

    pub unsafe extern "C" fn on_io_try_lock_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoTryLock { handle, lock_type })
    }

    pub unsafe extern "C" fn on_program_begin_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, program_name: OTF2_StringRef, number_of_arguments: u32, program_arguments: *const OTF2_StringRef ) -> OTF2_CallbackCode {
        let program_arguments = unsafe { std::slice::from_raw_parts(program_arguments, number_of_arguments as usize) }.to_vec();
        push_event!(queue, location, time, attributes, EventKind::ProgramBegin { program_name, program_arguments })
    }

    pub unsafe extern "C" fn on_program_end_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, exit_status: i64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ProgramEnd { exit_status })
    }

    pub unsafe extern "C" fn on_non_blocking_collective_request_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, request_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::NonBlockingCollectiveRequest { request_id })
    }

    pub unsafe extern "C" fn on_non_blocking_collective_complete_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, collective_op: OTF2_CollectiveOp, communicator: OTF2_CommRef, root: u32, size_sent: u64, size_received: u64, request_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::NonBlockingCollectiveComplete { collective_op, communicator, root, size_sent, size_received, request_id })
    }

    pub unsafe extern "C" fn on_comm_create_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, communicator: OTF2_CommRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::CommCreate { communicator })
    }

    pub unsafe extern "C" fn on_comm_destroy_event(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, communicator: OTF2_CommRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::CommDestroy { communicator })
    }
}
