#![allow(unused_variables)]

use crate::error::Status;
use crate::internal::*;
use crate::attribute::AttributeValue;
use std::ffi::CStr;

#[derive(Debug)]
pub struct Event {
    pub kind: EventKind,
    pub data: EventData,
}

impl Event {
    pub fn new(location: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: Vec<(OTF2_AttributeRef, AttributeValue)>, kind: EventKind) -> Self {
        Self {
            kind,
            data: EventData {
                location,
                time,
                attributes,
            },
        }
    }
}

#[derive(Debug)]
pub struct EventData {
    pub location: OTF2_LocationRef,
    pub time: OTF2_TimeStamp,
    pub attributes: Vec<(OTF2_AttributeRef, AttributeValue)>,
}

#[derive(Debug)]
pub enum EventKind {
    Unknown,
    BufferFlush {stop_time: OTF2_TimeStamp},
    MeasurementOnOff {measurement_mode: OTF2_MeasurementMode},
    Enter {region: OTF2_RegionRef},
    Leave {region: OTF2_RegionRef},
    MpiSend {receiver: u32, communicator: OTF2_CommRef, msg_tag: u32, msg_length: u64},
    MpiIsend {receiver: u32, communicator: OTF2_CommRef, msg_tag: u32, msg_length: u64, request_id: u64},
    MpiIsendComplete {request_id: u64},
    MpiIrecvRequest {request_id: u64},
    MpiRecv {sender: u32, communicator: OTF2_CommRef, msg_tag: u32, msg_length: u64},
    MpiIrecv {sender: u32, communicator: OTF2_CommRef, msg_tag: u32, msg_length: u64, request_id: u64},
    MpiRequestTest {request_id: u64},
    MpiRequestCancelled {request_id: u64},
    MpiCollectiveBegin ,
    MpiCollectiveEnd {collective_op: OTF2_CollectiveOp, communicator: OTF2_CommRef, root: u32, size_sent: u64, size_received: u64},
    OmpFork {number_of_requested_threads: u32},
    OmpJoin ,
    OmpAcquireLock {lock_id: u32, acquisition_order: u32},
    OmpReleaseLock {lock_id: u32, acquisition_order: u32},
    OmpTaskCreate {task_id: u64},
    OmpTaskSwitch {task_id: u64},
    OmpTaskComplete {task_id: u64},
    Metric {metric: OTF2_MetricRef, type_ids: Vec<OTF2_Type>, metric_values: Vec<OTF2_MetricValue>},
    ParameterString {parameter: OTF2_ParameterRef, string: OTF2_StringRef},
    ParameterInt {parameter: OTF2_ParameterRef, value: i64},
    ParameterUnsignedInt {parameter: OTF2_ParameterRef, value: u64},
    RmaWinCreate {win: OTF2_RmaWinRef},
    RmaWinDestroy {win: OTF2_RmaWinRef},
    RmaCollectiveBegin ,
    RmaCollectiveEnd {collective_op: OTF2_CollectiveOp, sync_level: OTF2_RmaSyncLevel, win: OTF2_RmaWinRef, root: u32, bytes_sent: u64, bytes_received: u64},
    RmaGroupSync {sync_level: OTF2_RmaSyncLevel, win: OTF2_RmaWinRef, group: OTF2_GroupRef},
    RmaRequestLock {win: OTF2_RmaWinRef, remote: u32, lock_id: u64, lock_type: OTF2_LockType},
    RmaAcquireLock {win: OTF2_RmaWinRef, remote: u32, lock_id: u64, lock_type: OTF2_LockType},
    RmaTryLock {win: OTF2_RmaWinRef, remote: u32, lock_id: u64, lock_type: OTF2_LockType},
    RmaReleaseLock {win: OTF2_RmaWinRef, remote: u32, lock_id: u64},
    RmaSync {win: OTF2_RmaWinRef, remote: u32, sync_type: OTF2_RmaSyncType},
    RmaWaitChange {win: OTF2_RmaWinRef},
    RmaPut {win: OTF2_RmaWinRef, remote: u32, bytes: u64, matching_id: u64},
    RmaGet {win: OTF2_RmaWinRef, remote: u32, bytes: u64, matching_id: u64},
    RmaAtomic {win: OTF2_RmaWinRef, remote: u32, type_: OTF2_RmaAtomicType, bytes_sent: u64, bytes_received: u64, matching_id: u64},
    RmaOpCompleteBlocking {win: OTF2_RmaWinRef, matching_id: u64},
    RmaOpCompleteNonBlocking {win: OTF2_RmaWinRef, matching_id: u64},
    RmaOpTest {win: OTF2_RmaWinRef, matching_id: u64},
    RmaOpCompleteRemote {win: OTF2_RmaWinRef, matching_id: u64},
    ThreadFork {model: OTF2_Paradigm, number_of_requested_threads: u32},
    ThreadJoin {model: OTF2_Paradigm},
    ThreadTeamBegin {thread_team: OTF2_CommRef},
    ThreadTeamEnd {thread_team: OTF2_CommRef},
    ThreadAcquireLock {model: OTF2_Paradigm, lock_id: u32, acquisition_order: u32},
    ThreadReleaseLock {model: OTF2_Paradigm, lock_id: u32, acquisition_order: u32},
    ThreadTaskCreate {thread_team: OTF2_CommRef, creating_thread: u32, generation_number: u32},
    ThreadTaskSwitch {thread_team: OTF2_CommRef, creating_thread: u32, generation_number: u32},
    ThreadTaskComplete {thread_team: OTF2_CommRef, creating_thread: u32, generation_number: u32},
    ThreadCreate {thread_contingent: OTF2_CommRef, sequence_count: u64},
    ThreadBegin {thread_contingent: OTF2_CommRef, sequence_count: u64},
    ThreadWait {thread_contingent: OTF2_CommRef, sequence_count: u64},
    ThreadEnd {thread_contingent: OTF2_CommRef, sequence_count: u64},
    CallingContextEnter {calling_context: OTF2_CallingContextRef, unwind_distance: u32},
    CallingContextLeave {calling_context: OTF2_CallingContextRef},
    CallingContextSample {calling_context: OTF2_CallingContextRef, unwind_distance: u32, interrupt_generator: OTF2_InterruptGeneratorRef},
    IoCreateHandle {handle: OTF2_IoHandleRef, mode: OTF2_IoAccessMode, creation_flags: OTF2_IoCreationFlag, status_flags: OTF2_IoStatusFlag},
    IoDestroyHandle {handle: OTF2_IoHandleRef},
    IoDuplicateHandle {old_handle: OTF2_IoHandleRef, new_handle: OTF2_IoHandleRef, status_flags: OTF2_IoStatusFlag},
    IoSeek {handle: OTF2_IoHandleRef, offset_request: i64, whence: OTF2_IoSeekOption, offset_result: u64},
    IoChangeStatusFlags {handle: OTF2_IoHandleRef, status_flags: OTF2_IoStatusFlag},
    IoDeleteFile {io_paradigm: OTF2_IoParadigmRef, file: OTF2_IoFileRef},
    IoOperationBegin {handle: OTF2_IoHandleRef, mode: OTF2_IoOperationMode, operation_flags: OTF2_IoOperationFlag, bytes_request: u64, matching_id: u64},
    IoOperationTest {handle: OTF2_IoHandleRef, matching_id: u64},
    IoOperationIssued {handle: OTF2_IoHandleRef, matching_id: u64},
    IoOperationComplete {handle: OTF2_IoHandleRef, bytes_result: u64, matching_id: u64},
    IoOperationCancelled {handle: OTF2_IoHandleRef, matching_id: u64},
    IoAcquireLock {handle: OTF2_IoHandleRef, lock_type: OTF2_LockType},
    IoReleaseLock {handle: OTF2_IoHandleRef, lock_type: OTF2_LockType},
    IoTryLock {handle: OTF2_IoHandleRef, lock_type: OTF2_LockType},
    ProgramBegin {program_name: OTF2_StringRef, program_arguments: Vec<OTF2_StringRef>},
    ProgramEnd {exit_status: i64},
    NonBlockingCollectiveRequest {request_id: u64},
    NonBlockingCollectiveComplete {collective_op: OTF2_CollectiveOp, communicator: OTF2_CommRef, root: u32, size_sent: u64, size_received: u64, request_id: u64},
    CommCreate {communicator: OTF2_CommRef},
    CommDestroy {communicator: OTF2_CommRef},
}

pub trait EventVisitor: std::fmt::Debug {
    fn visit_unknown_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)]) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_buffer_flush_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], stop_time: OTF2_TimeStamp ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_measurement_on_off_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], measurement_mode: OTF2_MeasurementMode ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_enter_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], region: OTF2_RegionRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_leave_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], region: OTF2_RegionRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_mpi_send_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], receiver: u32, communicator: OTF2_CommRef, msg_tag: u32, msg_length: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_mpi_isend_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], receiver: u32, communicator: OTF2_CommRef, msg_tag: u32, msg_length: u64, request_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_mpi_isend_complete_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], request_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_mpi_irecv_request_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], request_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_mpi_recv_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], sender: u32, communicator: OTF2_CommRef, msg_tag: u32, msg_length: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_mpi_irecv_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], sender: u32, communicator: OTF2_CommRef, msg_tag: u32, msg_length: u64, request_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_mpi_request_test_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], request_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_mpi_request_cancelled_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], request_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_mpi_collective_begin_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)] ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_mpi_collective_end_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], collective_op: OTF2_CollectiveOp, communicator: OTF2_CommRef, root: u32, size_sent: u64, size_received: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_omp_fork_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], number_of_requested_threads: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_omp_join_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)] ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_omp_acquire_lock_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], lock_id: u32, acquisition_order: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_omp_release_lock_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], lock_id: u32, acquisition_order: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_omp_task_create_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], task_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_omp_task_switch_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], task_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_omp_task_complete_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], task_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_metric_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], metric: OTF2_MetricRef, type_ids: &[OTF2_Type], metric_values: &[OTF2_MetricValue]) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_parameter_string_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], parameter: OTF2_ParameterRef, string: OTF2_StringRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_parameter_int_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], parameter: OTF2_ParameterRef, value: i64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_parameter_unsigned_int_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], parameter: OTF2_ParameterRef, value: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_rma_win_create_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], win: OTF2_RmaWinRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_rma_win_destroy_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], win: OTF2_RmaWinRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_rma_collective_begin_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)] ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_rma_collective_end_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], collective_op: OTF2_CollectiveOp, sync_level: OTF2_RmaSyncLevel, win: OTF2_RmaWinRef, root: u32, bytes_sent: u64, bytes_received: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_rma_group_sync_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], sync_level: OTF2_RmaSyncLevel, win: OTF2_RmaWinRef, group: OTF2_GroupRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_rma_request_lock_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], win: OTF2_RmaWinRef, remote: u32, lock_id: u64, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_rma_acquire_lock_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], win: OTF2_RmaWinRef, remote: u32, lock_id: u64, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_rma_try_lock_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], win: OTF2_RmaWinRef, remote: u32, lock_id: u64, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_rma_release_lock_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], win: OTF2_RmaWinRef, remote: u32, lock_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_rma_sync_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], win: OTF2_RmaWinRef, remote: u32, sync_type: OTF2_RmaSyncType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_rma_wait_change_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], win: OTF2_RmaWinRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_rma_put_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], win: OTF2_RmaWinRef, remote: u32, bytes: u64, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_rma_get_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], win: OTF2_RmaWinRef, remote: u32, bytes: u64, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_rma_atomic_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], win: OTF2_RmaWinRef, remote: u32, type_: OTF2_RmaAtomicType, bytes_sent: u64, bytes_received: u64, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_rma_op_complete_blocking_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], win: OTF2_RmaWinRef, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_rma_op_complete_non_blocking_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], win: OTF2_RmaWinRef, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_rma_op_test_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], win: OTF2_RmaWinRef, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_rma_op_complete_remote_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], win: OTF2_RmaWinRef, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_thread_fork_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], model: OTF2_Paradigm, number_of_requested_threads: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_thread_join_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], model: OTF2_Paradigm ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_thread_team_begin_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], thread_team: OTF2_CommRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_thread_team_end_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], thread_team: OTF2_CommRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_thread_acquire_lock_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], model: OTF2_Paradigm, lock_id: u32, acquisition_order: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_thread_release_lock_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], model: OTF2_Paradigm, lock_id: u32, acquisition_order: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_thread_task_create_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], thread_team: OTF2_CommRef, creating_thread: u32, generation_number: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_thread_task_switch_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], thread_team: OTF2_CommRef, creating_thread: u32, generation_number: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_thread_task_complete_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], thread_team: OTF2_CommRef, creating_thread: u32, generation_number: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_thread_create_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], thread_contingent: OTF2_CommRef, sequence_count: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_thread_begin_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], thread_contingent: OTF2_CommRef, sequence_count: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_thread_wait_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], thread_contingent: OTF2_CommRef, sequence_count: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_thread_end_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], thread_contingent: OTF2_CommRef, sequence_count: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_calling_context_enter_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], calling_context: OTF2_CallingContextRef, unwind_distance: u32 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_calling_context_leave_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], calling_context: OTF2_CallingContextRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_calling_context_sample_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], calling_context: OTF2_CallingContextRef, unwind_distance: u32, interrupt_generator: OTF2_InterruptGeneratorRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_io_create_handle_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], handle: OTF2_IoHandleRef, mode: OTF2_IoAccessMode, creation_flags: OTF2_IoCreationFlag, status_flags: OTF2_IoStatusFlag ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_io_destroy_handle_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], handle: OTF2_IoHandleRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_io_duplicate_handle_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], old_handle: OTF2_IoHandleRef, new_handle: OTF2_IoHandleRef, status_flags: OTF2_IoStatusFlag ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_io_seek_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], handle: OTF2_IoHandleRef, offset_request: i64, whence: OTF2_IoSeekOption, offset_result: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_io_change_status_flags_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], handle: OTF2_IoHandleRef, status_flags: OTF2_IoStatusFlag ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_io_delete_file_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], io_paradigm: OTF2_IoParadigmRef, file: OTF2_IoFileRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_io_operation_begin_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], handle: OTF2_IoHandleRef, mode: OTF2_IoOperationMode, operation_flags: OTF2_IoOperationFlag, bytes_request: u64, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_io_operation_test_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], handle: OTF2_IoHandleRef, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_io_operation_issued_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], handle: OTF2_IoHandleRef, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_io_operation_complete_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], handle: OTF2_IoHandleRef, bytes_result: u64, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_io_operation_cancelled_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], handle: OTF2_IoHandleRef, matching_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_io_acquire_lock_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], handle: OTF2_IoHandleRef, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_io_release_lock_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], handle: OTF2_IoHandleRef, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_io_try_lock_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], handle: OTF2_IoHandleRef, lock_type: OTF2_LockType ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_program_begin_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], program_name: OTF2_StringRef, program_arguments: &[OTF2_StringRef] ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_program_end_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], exit_status: i64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_non_blocking_collective_request_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], request_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_non_blocking_collective_complete_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], collective_op: OTF2_CollectiveOp, communicator: OTF2_CommRef, root: u32, size_sent: u64, size_received: u64, request_id: u64 ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_comm_create_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], communicator: OTF2_CommRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_comm_destroy_event(&mut self, location_id: OTF2_LocationRef, time: OTF2_TimeStamp, attributes: &[(OTF2_AttributeRef, AttributeValue)], communicator: OTF2_CommRef ) -> OTF2_CallbackCode {
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }
}
