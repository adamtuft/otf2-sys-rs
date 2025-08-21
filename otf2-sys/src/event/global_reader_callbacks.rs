#![allow(unused_variables)]

use crate::error::Status;
use crate::internal::*;
use crate::attribute::AttributeIterator;
use std::ffi::CStr;

use super::event_struct::{Event, EventKind};

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
        use event_queue_callbacks::*;
        unsafe {
            set_callbacks!(self,
                OTF2_GlobalEvtReaderCallbacks_SetUnknownCallback => unknown,
                OTF2_GlobalEvtReaderCallbacks_SetBufferFlushCallback => buffer_flush,
                OTF2_GlobalEvtReaderCallbacks_SetMeasurementOnOffCallback => measurement_on_off,
                OTF2_GlobalEvtReaderCallbacks_SetEnterCallback => enter,
                OTF2_GlobalEvtReaderCallbacks_SetLeaveCallback => leave,
                OTF2_GlobalEvtReaderCallbacks_SetMpiSendCallback => mpi_send,
                OTF2_GlobalEvtReaderCallbacks_SetMpiIsendCallback => mpi_isend,
                OTF2_GlobalEvtReaderCallbacks_SetMpiIsendCompleteCallback => mpi_isend_complete,
                OTF2_GlobalEvtReaderCallbacks_SetMpiIrecvRequestCallback => mpi_irecv_request,
                OTF2_GlobalEvtReaderCallbacks_SetMpiRecvCallback => mpi_recv,
                OTF2_GlobalEvtReaderCallbacks_SetMpiIrecvCallback => mpi_irecv,
                OTF2_GlobalEvtReaderCallbacks_SetMpiRequestTestCallback => mpi_request_test,
                OTF2_GlobalEvtReaderCallbacks_SetMpiRequestCancelledCallback => mpi_request_cancelled,
                OTF2_GlobalEvtReaderCallbacks_SetMpiCollectiveBeginCallback => mpi_collective_begin,
                OTF2_GlobalEvtReaderCallbacks_SetMpiCollectiveEndCallback => mpi_collective_end,
                OTF2_GlobalEvtReaderCallbacks_SetOmpForkCallback => omp_fork,
                OTF2_GlobalEvtReaderCallbacks_SetOmpJoinCallback => omp_join,
                OTF2_GlobalEvtReaderCallbacks_SetOmpAcquireLockCallback => omp_acquire_lock,
                OTF2_GlobalEvtReaderCallbacks_SetOmpReleaseLockCallback => omp_release_lock,
                OTF2_GlobalEvtReaderCallbacks_SetOmpTaskCreateCallback => omp_task_create,
                OTF2_GlobalEvtReaderCallbacks_SetOmpTaskSwitchCallback => omp_task_switch,
                OTF2_GlobalEvtReaderCallbacks_SetOmpTaskCompleteCallback => omp_task_complete,
                OTF2_GlobalEvtReaderCallbacks_SetMetricCallback => metric,
                OTF2_GlobalEvtReaderCallbacks_SetParameterStringCallback => parameter_string,
                OTF2_GlobalEvtReaderCallbacks_SetParameterIntCallback => parameter_int,
                OTF2_GlobalEvtReaderCallbacks_SetParameterUnsignedIntCallback => parameter_unsigned_int,
                OTF2_GlobalEvtReaderCallbacks_SetRmaWinCreateCallback => rma_win_create,
                OTF2_GlobalEvtReaderCallbacks_SetRmaWinDestroyCallback => rma_win_destroy,
                OTF2_GlobalEvtReaderCallbacks_SetRmaCollectiveBeginCallback => rma_collective_begin,
                OTF2_GlobalEvtReaderCallbacks_SetRmaCollectiveEndCallback => rma_collective_end,
                OTF2_GlobalEvtReaderCallbacks_SetRmaGroupSyncCallback => rma_group_sync,
                OTF2_GlobalEvtReaderCallbacks_SetRmaRequestLockCallback => rma_request_lock,
                OTF2_GlobalEvtReaderCallbacks_SetRmaAcquireLockCallback => rma_acquire_lock,
                OTF2_GlobalEvtReaderCallbacks_SetRmaTryLockCallback => rma_try_lock,
                OTF2_GlobalEvtReaderCallbacks_SetRmaReleaseLockCallback => rma_release_lock,
                OTF2_GlobalEvtReaderCallbacks_SetRmaSyncCallback => rma_sync,
                OTF2_GlobalEvtReaderCallbacks_SetRmaWaitChangeCallback => rma_wait_change,
                OTF2_GlobalEvtReaderCallbacks_SetRmaPutCallback => rma_put,
                OTF2_GlobalEvtReaderCallbacks_SetRmaGetCallback => rma_get,
                OTF2_GlobalEvtReaderCallbacks_SetRmaAtomicCallback => rma_atomic,
                OTF2_GlobalEvtReaderCallbacks_SetRmaOpCompleteBlockingCallback => rma_op_complete_blocking,
                OTF2_GlobalEvtReaderCallbacks_SetRmaOpCompleteNonBlockingCallback => rma_op_complete_non_blocking,
                OTF2_GlobalEvtReaderCallbacks_SetRmaOpTestCallback => rma_op_test,
                OTF2_GlobalEvtReaderCallbacks_SetRmaOpCompleteRemoteCallback => rma_op_complete_remote,
                OTF2_GlobalEvtReaderCallbacks_SetThreadForkCallback => thread_fork,
                OTF2_GlobalEvtReaderCallbacks_SetThreadJoinCallback => thread_join,
                OTF2_GlobalEvtReaderCallbacks_SetThreadTeamBeginCallback => thread_team_begin,
                OTF2_GlobalEvtReaderCallbacks_SetThreadTeamEndCallback => thread_team_end,
                OTF2_GlobalEvtReaderCallbacks_SetThreadAcquireLockCallback => thread_acquire_lock,
                OTF2_GlobalEvtReaderCallbacks_SetThreadReleaseLockCallback => thread_release_lock,
                OTF2_GlobalEvtReaderCallbacks_SetThreadTaskCreateCallback => thread_task_create,
                OTF2_GlobalEvtReaderCallbacks_SetThreadTaskSwitchCallback => thread_task_switch,
                OTF2_GlobalEvtReaderCallbacks_SetThreadTaskCompleteCallback => thread_task_complete,
                OTF2_GlobalEvtReaderCallbacks_SetThreadCreateCallback => thread_create,
                OTF2_GlobalEvtReaderCallbacks_SetThreadBeginCallback => thread_begin,
                OTF2_GlobalEvtReaderCallbacks_SetThreadWaitCallback => thread_wait,
                OTF2_GlobalEvtReaderCallbacks_SetThreadEndCallback => thread_end,
                OTF2_GlobalEvtReaderCallbacks_SetCallingContextEnterCallback => calling_context_enter,
                OTF2_GlobalEvtReaderCallbacks_SetCallingContextLeaveCallback => calling_context_leave,
                OTF2_GlobalEvtReaderCallbacks_SetCallingContextSampleCallback => calling_context_sample,
                OTF2_GlobalEvtReaderCallbacks_SetIoCreateHandleCallback => io_create_handle,
                OTF2_GlobalEvtReaderCallbacks_SetIoDestroyHandleCallback => io_destroy_handle,
                OTF2_GlobalEvtReaderCallbacks_SetIoDuplicateHandleCallback => io_duplicate_handle,
                OTF2_GlobalEvtReaderCallbacks_SetIoSeekCallback => io_seek,
                OTF2_GlobalEvtReaderCallbacks_SetIoChangeStatusFlagsCallback => io_change_status_flags,
                OTF2_GlobalEvtReaderCallbacks_SetIoDeleteFileCallback => io_delete_file,
                OTF2_GlobalEvtReaderCallbacks_SetIoOperationBeginCallback => io_operation_begin,
                OTF2_GlobalEvtReaderCallbacks_SetIoOperationTestCallback => io_operation_test,
                OTF2_GlobalEvtReaderCallbacks_SetIoOperationIssuedCallback => io_operation_issued,
                OTF2_GlobalEvtReaderCallbacks_SetIoOperationCompleteCallback => io_operation_complete,
                OTF2_GlobalEvtReaderCallbacks_SetIoOperationCancelledCallback => io_operation_cancelled,
                OTF2_GlobalEvtReaderCallbacks_SetIoAcquireLockCallback => io_acquire_lock,
                OTF2_GlobalEvtReaderCallbacks_SetIoReleaseLockCallback => io_release_lock,
                OTF2_GlobalEvtReaderCallbacks_SetIoTryLockCallback => io_try_lock,
                OTF2_GlobalEvtReaderCallbacks_SetProgramBeginCallback => program_begin,
                OTF2_GlobalEvtReaderCallbacks_SetProgramEndCallback => program_end,
                OTF2_GlobalEvtReaderCallbacks_SetNonBlockingCollectiveRequestCallback => non_blocking_collective_request,
                OTF2_GlobalEvtReaderCallbacks_SetNonBlockingCollectiveCompleteCallback => non_blocking_collective_complete,
                OTF2_GlobalEvtReaderCallbacks_SetCommCreateCallback => comm_create,
                OTF2_GlobalEvtReaderCallbacks_SetCommDestroyCallback => comm_destroy,
            );
        }
        Ok(())
    }
}

mod event_queue_callbacks {
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

    pub unsafe extern "C" fn unknown(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::Unknown{})
    }

    pub unsafe extern "C" fn buffer_flush(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, stop_time: OTF2_TimeStamp ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::BufferFlush { stop_time })
    }

    pub unsafe extern "C" fn measurement_on_off(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, measurement_mode: OTF2_MeasurementMode ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MeasurementOnOff { measurement_mode })
    }

    pub unsafe extern "C" fn enter(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, region: OTF2_RegionRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::Enter { region })
    }

    pub unsafe extern "C" fn leave(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, region: OTF2_RegionRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::Leave { region })
    }

    pub unsafe extern "C" fn mpi_send(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, receiver: u32, communicator: OTF2_CommRef, msg_tag: u32, msg_length: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MpiSend { receiver, communicator, msg_tag, msg_length })
    }

    pub unsafe extern "C" fn mpi_isend(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, receiver: u32, communicator: OTF2_CommRef, msg_tag: u32, msg_length: u64, request_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MpiIsend { receiver, communicator, msg_tag, msg_length, request_id })
    }

    pub unsafe extern "C" fn mpi_isend_complete(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, request_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MpiIsendComplete { request_id })
    }

    pub unsafe extern "C" fn mpi_irecv_request(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, request_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MpiIrecvRequest { request_id })
    }

    pub unsafe extern "C" fn mpi_recv(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, sender: u32, communicator: OTF2_CommRef, msg_tag: u32, msg_length: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MpiRecv { sender, communicator, msg_tag, msg_length })
    }

    pub unsafe extern "C" fn mpi_irecv(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, sender: u32, communicator: OTF2_CommRef, msg_tag: u32, msg_length: u64, request_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MpiIrecv { sender, communicator, msg_tag, msg_length, request_id })
    }

    pub unsafe extern "C" fn mpi_request_test(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, request_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MpiRequestTest { request_id })
    }

    pub unsafe extern "C" fn mpi_request_cancelled(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, request_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MpiRequestCancelled { request_id })
    }

    pub unsafe extern "C" fn mpi_collective_begin(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MpiCollectiveBegin{})
    }

    pub unsafe extern "C" fn mpi_collective_end(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, collective_op: OTF2_CollectiveOp, communicator: OTF2_CommRef, root: u32, size_sent: u64, size_received: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::MpiCollectiveEnd { collective_op, communicator, root, size_sent, size_received })
    }

    pub unsafe extern "C" fn omp_fork(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, number_of_requested_threads: u32 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::OmpFork { number_of_requested_threads })
    }

    pub unsafe extern "C" fn omp_join(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::OmpJoin{})
    }

    pub unsafe extern "C" fn omp_acquire_lock(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, lock_id: u32, acquisition_order: u32 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::OmpAcquireLock { lock_id, acquisition_order })
    }

    pub unsafe extern "C" fn omp_release_lock(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, lock_id: u32, acquisition_order: u32 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::OmpReleaseLock { lock_id, acquisition_order })
    }

    pub unsafe extern "C" fn omp_task_create(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, task_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::OmpTaskCreate { task_id })
    }

    pub unsafe extern "C" fn omp_task_switch(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, task_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::OmpTaskSwitch { task_id })
    }

    pub unsafe extern "C" fn omp_task_complete(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, task_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::OmpTaskComplete { task_id })
    }

    pub unsafe extern "C" fn metric(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, metric: OTF2_MetricRef, number_of_metrics: u8, type_ids: *const OTF2_Type, metric_values: *const OTF2_MetricValue ) -> OTF2_CallbackCode {
        let types = unsafe { std::slice::from_raw_parts(type_ids, number_of_metrics as usize) }.to_vec();
        let raw_values = unsafe { std::slice::from_raw_parts(metric_values, number_of_metrics as usize) }.to_vec();
        let values = types.into_iter().zip(raw_values).map(|(t, v)| MetricValue::new(t, v)).collect();
        push_event!(queue, location, time, attributes, EventKind::Metric { metric, values })
    }

    pub unsafe extern "C" fn parameter_string(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, parameter: OTF2_ParameterRef, string: OTF2_StringRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ParameterString { parameter, string })
    }

    pub unsafe extern "C" fn parameter_int(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, parameter: OTF2_ParameterRef, value: i64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ParameterInt { parameter, value })
    }

    pub unsafe extern "C" fn parameter_unsigned_int(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, parameter: OTF2_ParameterRef, value: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ParameterUnsignedInt { parameter, value })
    }

    pub unsafe extern "C" fn rma_win_create(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaWinCreate { win })
    }

    pub unsafe extern "C" fn rma_win_destroy(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaWinDestroy { win })
    }

    pub unsafe extern "C" fn rma_collective_begin(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaCollectiveBegin{})
    }

    pub unsafe extern "C" fn rma_collective_end(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, collective_op: OTF2_CollectiveOp, sync_level: OTF2_RmaSyncLevel, win: OTF2_RmaWinRef, root: u32, bytes_sent: u64, bytes_received: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaCollectiveEnd { collective_op, sync_level, win, root, bytes_sent, bytes_received })
    }

    pub unsafe extern "C" fn rma_group_sync(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, sync_level: OTF2_RmaSyncLevel, win: OTF2_RmaWinRef, group: OTF2_GroupRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaGroupSync { sync_level, win, group })
    }

    pub unsafe extern "C" fn rma_request_lock(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, lock_id: u64, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaRequestLock { win, remote, lock_id, lock_type })
    }

    pub unsafe extern "C" fn rma_acquire_lock(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, lock_id: u64, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaAcquireLock { win, remote, lock_id, lock_type })
    }

    pub unsafe extern "C" fn rma_try_lock(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, lock_id: u64, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaTryLock { win, remote, lock_id, lock_type })
    }

    pub unsafe extern "C" fn rma_release_lock(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, lock_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaReleaseLock { win, remote, lock_id })
    }

    pub unsafe extern "C" fn rma_sync(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, sync_type: OTF2_RmaSyncType ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaSync { win, remote, sync_type })
    }

    pub unsafe extern "C" fn rma_wait_change(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaWaitChange { win })
    }

    pub unsafe extern "C" fn rma_put(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, bytes: u64, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaPut { win, remote, bytes, matching_id })
    }

    pub unsafe extern "C" fn rma_get(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, bytes: u64, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaGet { win, remote, bytes, matching_id })
    }

    pub unsafe extern "C" fn rma_atomic(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, remote: u32, type_: OTF2_RmaAtomicType, bytes_sent: u64, bytes_received: u64, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaAtomic { win, remote, type_, bytes_sent, bytes_received, matching_id })
    }

    pub unsafe extern "C" fn rma_op_complete_blocking(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaOpCompleteBlocking { win, matching_id })
    }

    pub unsafe extern "C" fn rma_op_complete_non_blocking(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaOpCompleteNonBlocking { win, matching_id })
    }

    pub unsafe extern "C" fn rma_op_test(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaOpTest { win, matching_id })
    }

    pub unsafe extern "C" fn rma_op_complete_remote(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, win: OTF2_RmaWinRef, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::RmaOpCompleteRemote { win, matching_id })
    }

    pub unsafe extern "C" fn thread_fork(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, model: OTF2_Paradigm, number_of_requested_threads: u32 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadFork { model, number_of_requested_threads })
    }

    pub unsafe extern "C" fn thread_join(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, model: OTF2_Paradigm ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadJoin { model })
    }

    pub unsafe extern "C" fn thread_team_begin(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, thread_team: OTF2_CommRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadTeamBegin { thread_team })
    }

    pub unsafe extern "C" fn thread_team_end(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, thread_team: OTF2_CommRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadTeamEnd { thread_team })
    }

    pub unsafe extern "C" fn thread_acquire_lock(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, model: OTF2_Paradigm, lock_id: u32, acquisition_order: u32 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadAcquireLock { model, lock_id, acquisition_order })
    }

    pub unsafe extern "C" fn thread_release_lock(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, model: OTF2_Paradigm, lock_id: u32, acquisition_order: u32 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadReleaseLock { model, lock_id, acquisition_order })
    }

    pub unsafe extern "C" fn thread_task_create(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, thread_team: OTF2_CommRef, creating_thread: u32, generation_number: u32 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadTaskCreate { thread_team, creating_thread, generation_number })
    }

    pub unsafe extern "C" fn thread_task_switch(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, thread_team: OTF2_CommRef, creating_thread: u32, generation_number: u32 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadTaskSwitch { thread_team, creating_thread, generation_number })
    }

    pub unsafe extern "C" fn thread_task_complete(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, thread_team: OTF2_CommRef, creating_thread: u32, generation_number: u32 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadTaskComplete { thread_team, creating_thread, generation_number })
    }

    pub unsafe extern "C" fn thread_create(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, thread_contingent: OTF2_CommRef, sequence_count: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadCreate { thread_contingent, sequence_count })
    }

    pub unsafe extern "C" fn thread_begin(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, thread_contingent: OTF2_CommRef, sequence_count: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadBegin { thread_contingent, sequence_count })
    }

    pub unsafe extern "C" fn thread_wait(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, thread_contingent: OTF2_CommRef, sequence_count: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadWait { thread_contingent, sequence_count })
    }

    pub unsafe extern "C" fn thread_end(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, thread_contingent: OTF2_CommRef, sequence_count: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ThreadEnd { thread_contingent, sequence_count })
    }

    pub unsafe extern "C" fn calling_context_enter(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, calling_context: OTF2_CallingContextRef, unwind_distance: u32 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::CallingContextEnter { calling_context, unwind_distance })
    }

    pub unsafe extern "C" fn calling_context_leave(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, calling_context: OTF2_CallingContextRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::CallingContextLeave { calling_context })
    }

    pub unsafe extern "C" fn calling_context_sample(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, calling_context: OTF2_CallingContextRef, unwind_distance: u32, interrupt_generator: OTF2_InterruptGeneratorRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::CallingContextSample { calling_context, unwind_distance, interrupt_generator })
    }

    pub unsafe extern "C" fn io_create_handle(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, mode: OTF2_IoAccessMode, creation_flags: OTF2_IoCreationFlag, status_flags: OTF2_IoStatusFlag ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoCreateHandle { handle, mode, creation_flags, status_flags })
    }

    pub unsafe extern "C" fn io_destroy_handle(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoDestroyHandle { handle })
    }

    pub unsafe extern "C" fn io_duplicate_handle(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, old_handle: OTF2_IoHandleRef, new_handle: OTF2_IoHandleRef, status_flags: OTF2_IoStatusFlag ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoDuplicateHandle { old_handle, new_handle, status_flags })
    }

    pub unsafe extern "C" fn io_seek(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, offset_request: i64, whence: OTF2_IoSeekOption, offset_result: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoSeek { handle, offset_request, whence, offset_result })
    }

    pub unsafe extern "C" fn io_change_status_flags(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, status_flags: OTF2_IoStatusFlag ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoChangeStatusFlags { handle, status_flags })
    }

    pub unsafe extern "C" fn io_delete_file(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, io_paradigm: OTF2_IoParadigmRef, file: OTF2_IoFileRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoDeleteFile { io_paradigm, file })
    }

    pub unsafe extern "C" fn io_operation_begin(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, mode: OTF2_IoOperationMode, operation_flags: OTF2_IoOperationFlag, bytes_request: u64, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoOperationBegin { handle, mode, operation_flags, bytes_request, matching_id })
    }

    pub unsafe extern "C" fn io_operation_test(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoOperationTest { handle, matching_id })
    }

    pub unsafe extern "C" fn io_operation_issued(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoOperationIssued { handle, matching_id })
    }

    pub unsafe extern "C" fn io_operation_complete(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, bytes_result: u64, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoOperationComplete { handle, bytes_result, matching_id })
    }

    pub unsafe extern "C" fn io_operation_cancelled(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, matching_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoOperationCancelled { handle, matching_id })
    }

    pub unsafe extern "C" fn io_acquire_lock(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoAcquireLock { handle, lock_type })
    }

    pub unsafe extern "C" fn io_release_lock(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoReleaseLock { handle, lock_type })
    }

    pub unsafe extern "C" fn io_try_lock(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, handle: OTF2_IoHandleRef, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::IoTryLock { handle, lock_type })
    }

    pub unsafe extern "C" fn program_begin(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, program_name: OTF2_StringRef, number_of_arguments: u32, program_arguments: *const OTF2_StringRef ) -> OTF2_CallbackCode {
        let program_arguments = unsafe { std::slice::from_raw_parts(program_arguments, number_of_arguments as usize) }.to_vec();
        push_event!(queue, location, time, attributes, EventKind::ProgramBegin { program_name, program_arguments })
    }

    pub unsafe extern "C" fn program_end(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, exit_status: i64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::ProgramEnd { exit_status })
    }

    pub unsafe extern "C" fn non_blocking_collective_request(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, request_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::NonBlockingCollectiveRequest { request_id })
    }

    pub unsafe extern "C" fn non_blocking_collective_complete(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, collective_op: OTF2_CollectiveOp, communicator: OTF2_CommRef, root: u32, size_sent: u64, size_received: u64, request_id: u64 ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::NonBlockingCollectiveComplete { collective_op, communicator, root, size_sent, size_received, request_id })
    }

    pub unsafe extern "C" fn comm_create(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, communicator: OTF2_CommRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::CommCreate { communicator })
    }

    pub unsafe extern "C" fn comm_destroy(location: OTF2_LocationRef, time: OTF2_TimeStamp, queue: *mut c_void, attributes: *mut OTF2_AttributeList, communicator: OTF2_CommRef ) -> OTF2_CallbackCode {
        push_event!(queue, location, time, attributes, EventKind::CommDestroy { communicator })
    }
}
