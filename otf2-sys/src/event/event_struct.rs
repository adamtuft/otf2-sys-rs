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

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Event {{ location: {} kind: {:?} }}", self.data.location, self.kind)
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
