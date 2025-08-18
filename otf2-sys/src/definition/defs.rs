use crate::internal::*;
use crate::attribute::AttributeValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocationDef {
    pub name: OTF2_StringRef,
    pub location_type: OTF2_LocationType,
    pub num_events: u64,
    pub location_group: OTF2_LocationGroupRef,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttributeDef {
    pub name: OTF2_StringRef,
    pub description: OTF2_StringRef,
    pub kind: OTF2_Type,
}

#[derive(Debug, Clone, Copy)]
pub struct ClockPropertiesDef {
    pub timer_resolution: u64,
    pub global_offset: u64,
    pub trace_length: u64,
    pub realtime_timestamp: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct ParadigmDef {
    pub paradigm: OTF2_Paradigm,
    pub name: OTF2_StringRef,
    pub paradigm_class: OTF2_ParadigmClass,
}

#[derive(Debug, Clone, Copy)]
pub struct ParadigmPropertyDef {
    pub paradigm: OTF2_Paradigm,
    pub property: OTF2_ParadigmProperty,
    pub value: AttributeValue,
}


#[derive(Debug, Clone)]
pub struct IoParadigmDef {
    pub identification: OTF2_StringRef,
    pub name: OTF2_StringRef,
    pub io_paradigm_class: OTF2_IoParadigmClass,
    pub io_paradigm_flags: OTF2_IoParadigmFlag,
    pub properties: Vec<OTF2_IoParadigmProperty>,
    pub values: Vec<AttributeValue>,
}

#[derive(Debug, Clone, Copy)]
pub struct SystemTreeNodeDef {
    pub name: OTF2_StringRef,
    pub class_name: OTF2_StringRef,
    pub parent: Option<OTF2_SystemTreeNodeRef>,
}

#[derive(Debug, Clone, Copy)]
pub struct SystemTreeNodePropertyDef {
    pub system_tree_node: OTF2_SystemTreeNodeRef,
    pub name: OTF2_StringRef,
    pub value: AttributeValue,
}

#[derive(Debug, Clone, Copy)]
pub struct SystemTreeNodeDomainDef {
    pub system_tree_node: OTF2_SystemTreeNodeRef,
    pub system_tree_domain: OTF2_SystemTreeDomain,
}

#[derive(Debug, Clone, Copy)]
pub struct LocationGroupDef {
    pub name: OTF2_StringRef,
    pub location_group_type: OTF2_LocationGroupType,
    pub system_tree_parent: OTF2_SystemTreeNodeRef,
    pub creating_location_group: Option<OTF2_LocationGroupRef>,
}

#[derive(Debug, Clone, Copy)]
pub struct LocationGroupPropertyDef {
    pub location_group: OTF2_LocationGroupRef,
    pub name: OTF2_StringRef,
    pub value: AttributeValue,
}

#[derive(Debug, Clone, Copy)]
pub struct LocationPropertyDef {
    pub location: OTF2_LocationRef,
    pub name: OTF2_StringRef,
    pub value: AttributeValue,
}

#[derive(Debug, Clone, Copy)]
pub struct RegionDef {
    pub name: OTF2_StringRef,
    pub canonical_name: OTF2_StringRef,
    pub description: OTF2_StringRef,
    pub region_role: OTF2_RegionRole,
    pub paradigm: OTF2_Paradigm,
    pub region_flags: OTF2_RegionFlag,
    pub source_file: OTF2_StringRef,
    pub begin_line_number: u32,
    pub end_line_number: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct CallsiteDef {
    pub source_file: OTF2_StringRef,
    pub line_number: u32,
    pub entered_region: OTF2_RegionRef,
    pub left_region: OTF2_RegionRef,
}

#[derive(Debug, Clone, Copy)]
pub struct CallpathDef {
    pub parent: Option<OTF2_CallpathRef>,
    pub region: OTF2_RegionRef,
}

#[derive(Debug, Clone, Copy)]
pub struct CallpathParameterDef {
    pub callpath: OTF2_CallpathRef,
    pub parameter: OTF2_ParameterRef,
    pub value: AttributeValue,
}

#[derive(Debug, Clone, Copy)]
pub struct SourceCodeLocationDef {
    pub file: OTF2_StringRef,
    pub line_number: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct CallingContextDef {
    pub region: OTF2_RegionRef,
    pub source_code_location: OTF2_SourceCodeLocationRef,
    pub parent: Option<OTF2_CallingContextRef>,
}

#[derive(Debug, Clone, Copy)]
pub struct CallingContextPropertyDef {
    pub calling_context: OTF2_CallingContextRef,
    pub name: OTF2_StringRef,
    pub value: AttributeValue,
}

#[derive(Debug, Clone)]
pub struct GroupDef {
    pub name: OTF2_StringRef,
    pub group_type: OTF2_GroupType,
    pub paradigm: OTF2_Paradigm,
    pub group_flags: OTF2_GroupFlag,
    pub members: Vec<u64>,
}

#[derive(Debug, Clone, Copy)]
pub struct MetricMemberDef {
    pub name: OTF2_StringRef,
    pub description: OTF2_StringRef,
    pub metric_type: OTF2_MetricType,
    pub metric_mode: OTF2_MetricMode,
    pub value_type: OTF2_Type,
    pub base: OTF2_Base,
    pub exponent: i64,
    pub unit: OTF2_StringRef,
}

#[derive(Debug, Clone)]
pub struct MetricClassDef {
    pub metric_members: Vec<OTF2_MetricMemberRef>,
    pub metric_occurrence: OTF2_MetricOccurrence,
    pub recorder_kind: OTF2_RecorderKind,
}

#[derive(Debug, Clone, Copy)]
pub struct MetricInstanceDef {
    pub metric_class: OTF2_MetricRef,
    pub recorder: OTF2_LocationRef,
    pub metric_scope: OTF2_MetricScope,
    pub scope: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct MetricClassRecorderDef {
    pub metric_class: OTF2_MetricRef,
    pub recorder: OTF2_LocationRef,
}

#[derive(Debug, Clone, Copy)]
pub struct CommDef {
    pub name: OTF2_StringRef,
    pub group: OTF2_GroupRef,
    pub parent: Option<OTF2_CommRef>,
    pub flags: OTF2_CommFlag,
}

#[derive(Debug, Clone, Copy)]
pub struct InterCommDef {
    pub name: OTF2_StringRef,
    pub group_a: OTF2_GroupRef,
    pub group_b: OTF2_GroupRef,
    pub common_communicator: Option<OTF2_CommRef>,
    pub flags: OTF2_CommFlag,
}

#[derive(Debug, Clone, Copy)]
pub struct ParameterDef {
    pub name: OTF2_StringRef,
    pub parameter_type: OTF2_ParameterType,
}

#[derive(Debug, Clone, Copy)]
pub struct RmaWinDef {
    pub name: OTF2_StringRef,
    pub comm: OTF2_CommRef,
    pub flags: OTF2_RmaWinFlag,
}

#[derive(Debug, Clone, Copy)]
pub struct CartDimensionDef {
    pub name: OTF2_StringRef,
    pub size: u32,
    pub periodic: OTF2_CartPeriodicity,
}

#[derive(Debug, Clone)]
pub struct CartTopologyDef {
    pub name: OTF2_StringRef,
    pub communicator: OTF2_CommRef,
    pub dimensions: Vec<OTF2_CartDimensionRef>,
}

#[derive(Debug, Clone)]
pub struct CartCoordinateDef {
    pub topology: OTF2_CartTopologyRef,
    pub rank: u32,
    pub coordinates: Vec<u32>,
}

#[derive(Debug, Clone, Copy)]
pub struct InterruptGeneratorDef {
    pub name: OTF2_StringRef,
    pub interrupt_generator_mode: OTF2_InterruptGeneratorMode,
    pub base: OTF2_Base,
    pub exponent: i64,
    pub period: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct IoFilePropertyDef {
    pub io_file: OTF2_IoFileRef,
    pub name: OTF2_StringRef,
    pub value: AttributeValue,
}

#[derive(Debug, Clone, Copy)]
pub struct IoRegularFileDef {
    pub name: OTF2_StringRef,
    pub scope: OTF2_SystemTreeNodeRef,
}

#[derive(Debug, Clone, Copy)]
pub struct IoDirectoryDef {
    pub name: OTF2_StringRef,
    pub scope: OTF2_SystemTreeNodeRef,
}

#[derive(Debug, Clone, Copy)]
pub struct IoHandleDef {
    pub name: OTF2_StringRef,
    pub file: OTF2_IoFileRef,
    pub io_paradigm: OTF2_IoParadigmRef,
    pub io_handle_flags: OTF2_IoHandleFlag,
    pub comm: Option<OTF2_CommRef>,
    pub parent: Option<OTF2_IoHandleRef>,
}

#[derive(Debug, Clone, Copy)]
pub struct IoPreCreatedHandleStateDef {
    pub io_handle: OTF2_IoHandleRef,
    pub mode: OTF2_IoAccessMode,
    pub status_flags: OTF2_IoStatusFlag,
}

/// Stores definitions from a trace file. Each variant stores the data provided by the corresponding
/// callback function which reports the particular definition.
#[derive(Debug)]
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
        paradigm: OTF2_Paradigm,
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
        system_tree_node: OTF2_SystemTreeNodeRef,
        value: SystemTreeNodePropertyDef,
    },
    SystemTreeNodeDomain {
        system_tree_node: OTF2_SystemTreeNodeRef,
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
        location_group: OTF2_LocationGroupRef,
        value: LocationGroupPropertyDef,
    },
    LocationProperty {
        location: OTF2_LocationRef,
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
        callpath: OTF2_CallpathRef,
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
        calling_context: OTF2_CallingContextRef,
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
        metric_class: OTF2_MetricRef,
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
        topology: OTF2_CartTopologyRef,
        value: CartCoordinateDef,
    },
    InterruptGenerator {
        defn: OTF2_InterruptGeneratorRef,
        value: InterruptGeneratorDef,
    },
    IoFileProperty {
        io_file: OTF2_IoFileRef,
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
        io_handle: OTF2_IoHandleRef,
        value: IoPreCreatedHandleStateDef,
    },
}
