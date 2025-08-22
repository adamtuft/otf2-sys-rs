use crate::internal::*;
use crate::attribute::AttributeValue;

macro_rules! for_each_definition {
    ($macro:ident) => {
        $macro!(
            name: String, reftype: OTF2_StringRef, type: String;
            name: Attribute, reftype: OTF2_AttributeRef, struct: AttributeDef { name: OTF2_StringRef, description: OTF2_StringRef, kind: OTF2_Type};
            name: ClockProperties, struct: ClockPropertiesDef { timer_resolution: u64, global_offset: u64, trace_length: u64, realtime_timestamp: u64};
            name: Paradigm, reftype: OTF2_Paradigm, struct: ParadigmDef { paradigm: OTF2_Paradigm, name: OTF2_StringRef, paradigm_class: OTF2_ParadigmClass};
            name: ParadigmProperty, reftype: OTF2_Paradigm, struct: ParadigmPropertyDef { paradigm: OTF2_Paradigm, property: OTF2_ParadigmProperty, value: AttributeValue};
            name: IoParadigm, reftype: OTF2_IoParadigmRef, struct: IoParadigmDef { identification: OTF2_StringRef, name: OTF2_StringRef, io_paradigm_class: OTF2_IoParadigmClass, io_paradigm_flags: OTF2_IoParadigmFlag, properties: Vec<OTF2_IoParadigmProperty>, values: Vec<AttributeValue>};
            name: SystemTreeNode, reftype: OTF2_SystemTreeNodeRef, struct: SystemTreeNodeDef { name: OTF2_StringRef, class_name: OTF2_StringRef, parent: Option<OTF2_SystemTreeNodeRef>};
            name: SystemTreeNodeProperty, reftype: OTF2_SystemTreeNodeRef, struct: SystemTreeNodePropertyDef { system_tree_node: OTF2_SystemTreeNodeRef, name: OTF2_StringRef, value: AttributeValue};
            name: SystemTreeNodeDomain, reftype: OTF2_SystemTreeNodeRef, struct: SystemTreeNodeDomainDef { system_tree_node: OTF2_SystemTreeNodeRef, system_tree_domain: OTF2_SystemTreeDomain};
            name: Location, reftype: OTF2_LocationRef, struct: LocationDef { name: OTF2_StringRef, location_type: OTF2_LocationType, num_events: u64, location_group: OTF2_LocationGroupRef};
            name: LocationGroup, reftype: OTF2_LocationGroupRef, struct: LocationGroupDef { name: OTF2_StringRef, location_group_type: OTF2_LocationGroupType, system_tree_parent: OTF2_SystemTreeNodeRef, creating_location_group: Option<OTF2_LocationGroupRef>};
            name: LocationGroupProperty, reftype: OTF2_LocationGroupRef, struct: LocationGroupPropertyDef { location_group: OTF2_LocationGroupRef, name: OTF2_StringRef, value: AttributeValue};
            name: LocationProperty, reftype: OTF2_LocationRef, struct: LocationPropertyDef { location: OTF2_LocationRef, name: OTF2_StringRef, value: AttributeValue};
            name: Region, reftype: OTF2_RegionRef, struct: RegionDef { name: OTF2_StringRef, canonical_name: OTF2_StringRef, description: OTF2_StringRef, region_role: OTF2_RegionRole, paradigm: OTF2_Paradigm, region_flags: OTF2_RegionFlag, source_file: OTF2_StringRef, begin_line_number: u32, end_line_number: u32};
            name: Callsite, reftype: OTF2_CallsiteRef, struct: CallsiteDef { source_file: OTF2_StringRef, line_number: u32, entered_region: OTF2_RegionRef, left_region: OTF2_RegionRef};
            name: Callpath, reftype: OTF2_CallpathRef, struct: CallpathDef { parent: Option<OTF2_CallpathRef>, region: OTF2_RegionRef};
            name: CallpathParameter, reftype: OTF2_CallpathRef, struct: CallpathParameterDef { callpath: OTF2_CallpathRef, parameter: OTF2_ParameterRef, value: AttributeValue};
            name: SourceCodeLocation, reftype: OTF2_SourceCodeLocationRef, struct: SourceCodeLocationDef { file: OTF2_StringRef, line_number: u32};
            name: CallingContext, reftype: OTF2_CallingContextRef, struct: CallingContextDef { region: OTF2_RegionRef, source_code_location: OTF2_SourceCodeLocationRef, parent: Option<OTF2_CallingContextRef>};
            name: CallingContextProperty, reftype: OTF2_CallingContextRef, struct: CallingContextPropertyDef { calling_context: OTF2_CallingContextRef, name: OTF2_StringRef, value: AttributeValue};
            name: Group, reftype: OTF2_GroupRef, struct: GroupDef { name: OTF2_StringRef, group_type: OTF2_GroupType, paradigm: OTF2_Paradigm, group_flags: OTF2_GroupFlag, members: Vec<u64>};
            name: MetricMember, reftype: OTF2_MetricMemberRef, struct: MetricMemberDef { name: OTF2_StringRef, description: OTF2_StringRef, metric_type: OTF2_MetricType, metric_mode: OTF2_MetricMode, value_type: OTF2_Type, base: OTF2_Base, exponent: i64, unit: OTF2_StringRef};
            name: MetricClass, reftype: OTF2_MetricRef, struct: MetricClassDef { metric_members: Vec<OTF2_MetricMemberRef>, metric_occurrence: OTF2_MetricOccurrence, recorder_kind: OTF2_RecorderKind};
            name: MetricInstance, reftype: OTF2_MetricRef, struct: MetricInstanceDef { metric_class: OTF2_MetricRef, recorder: OTF2_LocationRef, metric_scope: OTF2_MetricScope, scope: u64};
            name: MetricClassRecorder, reftype: OTF2_MetricRef, struct: MetricClassRecorderDef { metric_class: OTF2_MetricRef, recorder: OTF2_LocationRef};
            name: Comm, reftype: OTF2_CommRef, struct: CommDef { name: OTF2_StringRef, group: OTF2_GroupRef, parent: Option<OTF2_CommRef>, flags: OTF2_CommFlag};
            name: InterComm, reftype: OTF2_CommRef, struct: InterCommDef { name: OTF2_StringRef, group_a: OTF2_GroupRef, group_b: OTF2_GroupRef, common_communicator: Option<OTF2_CommRef>, flags: OTF2_CommFlag};
            name: Parameter, reftype: OTF2_ParameterRef, struct: ParameterDef { name: OTF2_StringRef, parameter_type: OTF2_ParameterType};
            name: RmaWin, reftype: OTF2_RmaWinRef, struct: RmaWinDef { name: OTF2_StringRef, comm: OTF2_CommRef, flags: OTF2_RmaWinFlag};
            name: CartDimension, reftype: OTF2_CartDimensionRef, struct: CartDimensionDef { name: OTF2_StringRef, size: u32, periodic: OTF2_CartPeriodicity};
            name: CartTopology, reftype: OTF2_CartTopologyRef, struct: CartTopologyDef { name: OTF2_StringRef, communicator: OTF2_CommRef, dimensions: Vec<OTF2_CartDimensionRef>};
            name: CartCoordinate, reftype: OTF2_CartTopologyRef, struct: CartCoordinateDef { topology: OTF2_CartTopologyRef, rank: u32, coordinates: Vec<u32>};
            name: InterruptGenerator, reftype: OTF2_InterruptGeneratorRef, struct: InterruptGeneratorDef { name: OTF2_StringRef, interrupt_generator_mode: OTF2_InterruptGeneratorMode, base: OTF2_Base, exponent: i64, period: u64};
            name: IoFileProperty, reftype: OTF2_IoFileRef, struct: IoFilePropertyDef { io_file: OTF2_IoFileRef, name: OTF2_StringRef, value: AttributeValue};
            name: IoRegularFile, reftype: OTF2_IoFileRef, struct: IoRegularFileDef { name: OTF2_StringRef, scope: OTF2_SystemTreeNodeRef};
            name: IoDirectory, reftype: OTF2_IoFileRef, struct: IoDirectoryDef { name: OTF2_StringRef, scope: OTF2_SystemTreeNodeRef};
            name: IoHandle, reftype: OTF2_IoHandleRef, struct: IoHandleDef { name: OTF2_StringRef, file: OTF2_IoFileRef, io_paradigm: OTF2_IoParadigmRef, io_handle_flags: OTF2_IoHandleFlag, comm: Option<OTF2_CommRef>, parent: Option<OTF2_IoHandleRef>};
            name: IoPreCreatedHandleState, reftype: OTF2_IoHandleRef, struct: IoPreCreatedHandleStateDef { io_handle: OTF2_IoHandleRef, mode: OTF2_IoAccessMode, status_flags: OTF2_IoStatusFlag};
        );
    }
}

macro_rules! define_value_struct {
    () => {};
    ( name: $name:ident, $(reftype: $reftype:ty ,)? struct: $struct:ident { $( $field:ident: $ty:ty ),* }; $($rest:tt)* ) => {
        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $struct {
            $(pub $field: $ty),*
        }
        define_value_struct!( $($rest)* );
    };
    ( name: $name:ident, $(reftype: $reftype:ty ,)? type: $type:ty; $($rest:tt)* ) => {
        define_value_struct!( $($rest)* );
    };
}

for_each_definition!(define_value_struct);

/// Stores definitions from a trace file. Each variant stores the data provided by the corresponding
/// callback function which reports the particular definition.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Definition {
    String {
        defn: OTF2_StringRef,
        value: String,
    },
    Attribute {
        defn: OTF2_AttributeRef,
        value: AttributeDef,
    },
    ClockProperties {
        value: ClockPropertiesDef,
    },
    Paradigm {
        defn: OTF2_Paradigm,
        value: ParadigmDef,
    },
    ParadigmProperty {
        defn: OTF2_Paradigm,
        value: ParadigmPropertyDef,
    },
    IoParadigm {
        defn: OTF2_IoParadigmRef,
        value: IoParadigmDef,
    },
    SystemTreeNode {
        defn: OTF2_SystemTreeNodeRef,
        value: SystemTreeNodeDef,
    },
    SystemTreeNodeProperty {
        defn: OTF2_SystemTreeNodeRef,
        value: SystemTreeNodePropertyDef,
    },
    SystemTreeNodeDomain {
        defn: OTF2_SystemTreeNodeRef,
        value: SystemTreeNodeDomainDef,
    },
    LocationGroup {
        defn: OTF2_LocationGroupRef,
        value: LocationGroupDef,
    },
    Location {
        defn: OTF2_LocationRef,
        value: LocationDef,
    },
    LocationGroupProperty {
        defn: OTF2_LocationGroupRef,
        value: LocationGroupPropertyDef,
    },
    LocationProperty {
        defn: OTF2_LocationRef,
        value: LocationPropertyDef,
    },
    Region {
        defn: OTF2_RegionRef,
        value: RegionDef,
    },
    Callsite {
        defn: OTF2_CallsiteRef,
        value: CallsiteDef,
    },
    Callpath {
        defn: OTF2_CallpathRef,
        value: CallpathDef,
    },
    CallpathParameter {
        defn: OTF2_CallpathRef,
        value: CallpathParameterDef,
    },
    SourceCodeLocation {
        defn: OTF2_SourceCodeLocationRef,
        value: SourceCodeLocationDef,
    },
    CallingContext {
        defn: OTF2_CallingContextRef,
        value: CallingContextDef,
    },
    CallingContextProperty {
        defn: OTF2_CallingContextRef,
        value: CallingContextPropertyDef,
    },
    Group {
        defn: OTF2_GroupRef,
        value: GroupDef,
    },
    MetricMember {
        defn: OTF2_MetricMemberRef,
        value: MetricMemberDef,
    },
    MetricClass {
        defn: OTF2_MetricRef,
        value: MetricClassDef,
    },
    MetricInstance {
        defn: OTF2_MetricRef,
        value: MetricInstanceDef,
    },
    MetricClassRecorder {
        defn: OTF2_MetricRef,
        value: MetricClassRecorderDef,
    },
    Comm {
        defn: OTF2_CommRef,
        value: CommDef,
    },
    InterComm {
        defn: OTF2_CommRef,
        value: InterCommDef,
    },
    Parameter {
        defn: OTF2_ParameterRef,
        value: ParameterDef,
    },
    RmaWin {
        defn: OTF2_RmaWinRef,
        value: RmaWinDef,
    },
    CartDimension {
        defn: OTF2_CartDimensionRef,
        value: CartDimensionDef,
    },
    CartTopology {
        defn: OTF2_CartTopologyRef,
        value: CartTopologyDef,
    },
    CartCoordinate {
        defn: OTF2_CartTopologyRef,
        value: CartCoordinateDef,
    },
    InterruptGenerator {
        defn: OTF2_InterruptGeneratorRef,
        value: InterruptGeneratorDef,
    },
    IoFileProperty {
        defn: OTF2_IoFileRef,
        value: IoFilePropertyDef,
    },
    IoRegularFile {
        defn: OTF2_IoFileRef,
        value: IoRegularFileDef,
    },
    IoDirectory {
        defn: OTF2_IoFileRef,
        value: IoDirectoryDef,
    },
    IoHandle {
        defn: OTF2_IoHandleRef,
        value: IoHandleDef,
    },
    IoPreCreatedHandleState {
        defn: OTF2_IoHandleRef,
        value: IoPreCreatedHandleStateDef,
    },
}
