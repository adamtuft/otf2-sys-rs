use crate::error::Status;
use crate::internal::*;
use std::ffi::CStr;

use crate::definition::DefinitionVisitor;

/// A visitor that prints all definition information to stderr
pub struct PrintingDefinitionVisitor;

impl PrintingDefinitionVisitor {
    pub fn new() -> Self {
        Self
    }
}

impl DefinitionVisitor for PrintingDefinitionVisitor {
    fn visit_string(&mut self, defn: OTF2_StringRef, value: &CStr) -> OTF2_CallbackCode {
        eprintln!(
            "visit_string: defn: {}, value: {}",
            defn,
            value.to_string_lossy()
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_location(
        &mut self,
        defn: OTF2_LocationRef,
        name: OTF2_StringRef,
        location_type: OTF2_LocationType,
        num_events: u64,
        location_group: OTF2_LocationGroupRef,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_location: defn: {}, name: {}, location_type: {}, num_events: {}, location_group: {}",
            defn, name, location_type, num_events, location_group
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_clock_properties(
        &mut self,
        timer_resolution: u64,
        global_offset: u64,
        trace_length: u64,
        realtime_timestamp: u64,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_clock_properties: timer_resolution: {}, global_offset: {}, trace_length: {}, realtime_timestamp: {}",
            timer_resolution, global_offset, trace_length, realtime_timestamp
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_paradigm(
        &mut self,
        paradigm: OTF2_Paradigm,
        name: OTF2_StringRef,
        paradigm_class: OTF2_ParadigmClass,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_paradigm: paradigm: {}, name: {}, paradigm_class: {}",
            paradigm, name, paradigm_class
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_paradigm_property(
        &mut self,
        paradigm: OTF2_Paradigm,
        property: OTF2_ParadigmProperty,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_paradigm_property: paradigm: {}, property: {}, type: {}, value: <attr_value>",
            paradigm, property, type_
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_io_paradigm(
        &mut self,
        defn: OTF2_IoParadigmRef,
        identification: OTF2_StringRef,
        name: OTF2_StringRef,
        io_paradigm_class: OTF2_IoParadigmClass,
        io_paradigm_flags: OTF2_IoParadigmFlag,
        properties: &[OTF2_IoParadigmProperty],
        types: &[OTF2_Type],
        values: &[OTF2_AttributeValue],
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_io_paradigm: defn: {}, identification: {}, name: {}, io_paradigm_class: {}, io_paradigm_flags: {}, properties: {} items",
            defn, identification, name, io_paradigm_class, io_paradigm_flags, properties.len()
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_attribute(
        &mut self,
        defn: OTF2_AttributeRef,
        name: OTF2_StringRef,
        description: OTF2_StringRef,
        type_: OTF2_Type,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_attribute: defn: {}, name: {}, description: {}, type: {}",
            defn, name, description, type_
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_system_tree_node(
        &mut self,
        defn: OTF2_SystemTreeNodeRef,
        name: OTF2_StringRef,
        class_name: OTF2_StringRef,
        parent: Option<OTF2_SystemTreeNodeRef>,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_system_tree_node: defn: {}, name: {}, class_name: {}, parent: {:?}",
            defn, name, class_name, parent
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_system_tree_node_property(
        &mut self,
        system_tree_node: OTF2_SystemTreeNodeRef,
        name: OTF2_StringRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_system_tree_node_property: system_tree_node: {}, name: {}, type: {}, value: <attr_value>",
            system_tree_node, name, type_
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_system_tree_node_domain(
        &mut self,
        system_tree_node: OTF2_SystemTreeNodeRef,
        system_tree_domain: OTF2_SystemTreeDomain,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_system_tree_node_domain: system_tree_node: {}, system_tree_domain: {}",
            system_tree_node, system_tree_domain
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_location_group(
        &mut self,
        defn: OTF2_LocationGroupRef,
        name: OTF2_StringRef,
        location_group_type: OTF2_LocationGroupType,
        system_tree_parent: OTF2_SystemTreeNodeRef,
        creating_location_group: Option<OTF2_LocationGroupRef>,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_location_group: defn: {}, name: {}, location_group_type: {}, system_tree_parent: {}, creating_location_group: {:?}",
            defn, name, location_group_type, system_tree_parent, creating_location_group
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_location_group_property(
        &mut self,
        location_group: OTF2_LocationGroupRef,
        name: OTF2_StringRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_location_group_property: location_group: {}, name: {}, type: {}, value: <attr_value>",
            location_group, name, type_
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_location_property(
        &mut self,
        location: OTF2_LocationRef,
        name: OTF2_StringRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_location_property: location: {}, name: {}, type: {}, value: <attr_value>",
            location, name, type_
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_region(
        &mut self,
        defn: OTF2_RegionRef,
        name: OTF2_StringRef,
        canonical_name: OTF2_StringRef,
        description: OTF2_StringRef,
        region_role: OTF2_RegionRole,
        paradigm: OTF2_Paradigm,
        region_flags: OTF2_RegionFlag,
        source_file: OTF2_StringRef,
        begin_line_number: u32,
        end_line_number: u32,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_region: defn: {}, name: {}, canonical_name: {}, description: {}, region_role: {}, paradigm: {}, region_flags: {}, source_file: {}, begin_line_number: {}, end_line_number: {}",
            defn, name, canonical_name, description, region_role, paradigm, region_flags, source_file, begin_line_number, end_line_number
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_callsite(
        &mut self,
        defn: OTF2_CallsiteRef,
        source_file: OTF2_StringRef,
        line_number: u32,
        entered_region: OTF2_RegionRef,
        left_region: OTF2_RegionRef,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_callsite: defn: {}, source_file: {}, line_number: {}, entered_region: {}, left_region: {}",
            defn, source_file, line_number, entered_region, left_region
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_callpath(
        &mut self,
        defn: OTF2_CallpathRef,
        parent: Option<OTF2_CallpathRef>,
        region: OTF2_RegionRef,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_callpath: defn: {}, parent: {:?}, region: {}",
            defn, parent, region
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_callpath_parameter(
        &mut self,
        callpath: OTF2_CallpathRef,
        parameter: OTF2_ParameterRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_callpath_parameter: callpath: {}, parameter: {}, type: {}, value: <attr_value>",
            callpath, parameter, type_
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_source_code_location(
        &mut self,
        defn: OTF2_SourceCodeLocationRef,
        file: OTF2_StringRef,
        line_number: u32,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_source_code_location: defn: {}, file: {}, line_number: {}",
            defn, file, line_number
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_calling_context(
        &mut self,
        defn: OTF2_CallingContextRef,
        region: OTF2_RegionRef,
        source_code_location: OTF2_SourceCodeLocationRef,
        parent: Option<OTF2_CallingContextRef>,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_calling_context: defn: {}, region: {}, source_code_location: {}, parent: {:?}",
            defn, region, source_code_location, parent
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_calling_context_property(
        &mut self,
        calling_context: OTF2_CallingContextRef,
        name: OTF2_StringRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_calling_context_property: calling_context: {}, name: {}, type: {}, value: <attr_value>",
            calling_context, name, type_
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_group(
        &mut self,
        defn: OTF2_GroupRef,
        name: OTF2_StringRef,
        group_type: OTF2_GroupType,
        paradigm: OTF2_Paradigm,
        group_flags: OTF2_GroupFlag,
        members: &[u64],
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_group: defn: {}, name: {}, group_type: {}, paradigm: {}, group_flags: {}, members: {} items",
            defn, name, group_type, paradigm, group_flags, members.len()
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_metric_member(
        &mut self,
        defn: OTF2_MetricMemberRef,
        name: OTF2_StringRef,
        description: OTF2_StringRef,
        metric_type: OTF2_MetricType,
        metric_mode: OTF2_MetricMode,
        value_type: OTF2_Type,
        base: OTF2_Base,
        exponent: i64,
        unit: OTF2_StringRef,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_metric_member: defn: {}, name: {}, description: {}, metric_type: {}, metric_mode: {}, value_type: {}, base: {}, exponent: {}, unit: {}",
            defn, name, description, metric_type, metric_mode, value_type, base, exponent, unit
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_metric_class(
        &mut self,
        defn: OTF2_MetricRef,
        metric_members: &[OTF2_MetricMemberRef],
        metric_occurrence: OTF2_MetricOccurrence,
        recorder_kind: OTF2_RecorderKind,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_metric_class: defn: {}, metric_members: {} items, metric_occurrence: {}, recorder_kind: {}",
            defn, metric_members.len(), metric_occurrence, recorder_kind
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_metric_instance(
        &mut self,
        defn: OTF2_MetricRef,
        metric_class: OTF2_MetricRef,
        recorder: OTF2_LocationRef,
        metric_scope: OTF2_MetricScope,
        scope: u64,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_metric_instance: defn: {}, metric_class: {}, recorder: {}, metric_scope: {}, scope: {}",
            defn, metric_class, recorder, metric_scope, scope
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_metric_class_recorder(
        &mut self,
        metric_class: OTF2_MetricRef,
        recorder: OTF2_LocationRef,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_metric_class_recorder: metric_class: {}, recorder: {}",
            metric_class, recorder
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_comm(
        &mut self,
        defn: OTF2_CommRef,
        name: OTF2_StringRef,
        group: OTF2_GroupRef,
        parent: Option<OTF2_CommRef>,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_comm: defn: {}, name: {}, group: {}, parent: {:?}",
            defn, name, group, parent
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_inter_comm(
        &mut self,
        defn: OTF2_CommRef,
        name: OTF2_StringRef,
        group_a: OTF2_GroupRef,
        group_b: OTF2_GroupRef,
        common_communicator: Option<OTF2_CommRef>,
        flags: OTF2_CommFlag,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_inter_comm: defn: {}, name: {}, group_a: {}, group_b: {}, common_communicator: {:?}, flags: {}",
            defn, name, group_a, group_b, common_communicator, flags
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_parameter(
        &mut self,
        defn: OTF2_ParameterRef,
        name: OTF2_StringRef,
        parameter_type: OTF2_ParameterType,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_parameter: defn: {}, name: {}, parameter_type: {}",
            defn, name, parameter_type
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_rma_win(
        &mut self,
        defn: OTF2_RmaWinRef,
        name: OTF2_StringRef,
        comm: OTF2_CommRef,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_rma_win: defn: {}, name: {}, comm: {}",
            defn, name, comm
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_cart_dimension(
        &mut self,
        defn: OTF2_CartDimensionRef,
        name: OTF2_StringRef,
        size: u32,
        periodic: OTF2_CartPeriodicity,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_cart_dimension: defn: {}, name: {}, size: {}, periodic: {}",
            defn, name, size, periodic
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_cart_topology(
        &mut self,
        defn: OTF2_CartTopologyRef,
        name: OTF2_StringRef,
        communicator: OTF2_CommRef,
        dimensions: &[OTF2_CartDimensionRef],
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_cart_topology: defn: {}, name: {}, communicator: {}, dimensions: {} items",
            defn, name, communicator, dimensions.len()
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_cart_coordinate(
        &mut self,
        topology: OTF2_CartTopologyRef,
        rank: u32,
        coordinates: &[u32],
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_cart_coordinate: topology: {}, rank: {}, coordinates: {} items",
            topology, rank, coordinates.len()
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_interrupt_generator(
        &mut self,
        defn: OTF2_InterruptGeneratorRef,
        name: OTF2_StringRef,
        interrupt_generator_mode: OTF2_InterruptGeneratorMode,
        base: OTF2_Base,
        exponent: i64,
        period: u64,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_interrupt_generator: defn: {}, name: {}, interrupt_generator_mode: {}, base: {}, exponent: {}, period: {}",
            defn, name, interrupt_generator_mode, base, exponent, period
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_io_file_property(
        &mut self,
        io_file: OTF2_IoFileRef,
        name: OTF2_StringRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_io_file_property: io_file: {}, name: {}, type: {}, value: <attr_value>",
            io_file, name, type_
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_io_regular_file(
        &mut self,
        defn: OTF2_IoFileRef,
        name: OTF2_StringRef,
        scope: OTF2_SystemTreeNodeRef,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_io_regular_file: defn: {}, name: {}, scope: {}",
            defn, name, scope
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_io_directory(
        &mut self,
        defn: OTF2_IoFileRef,
        name: OTF2_StringRef,
        scope: OTF2_SystemTreeNodeRef,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_io_directory: defn: {}, name: {}, scope: {}",
            defn, name, scope
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_io_handle(
        &mut self,
        defn: OTF2_IoHandleRef,
        name: OTF2_StringRef,
        file: OTF2_IoFileRef,
        io_paradigm: OTF2_IoParadigmRef,
        io_handle_flags: OTF2_IoHandleFlag,
        comm: Option<OTF2_CommRef>,
        parent: Option<OTF2_IoHandleRef>,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_io_handle: defn: {}, name: {}, file: {}, io_paradigm: {}, io_handle_flags: {}, comm: {:?}, parent: {:?}",
            defn, name, file, io_paradigm, io_handle_flags, comm, parent
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_io_pre_created_handle_state(
        &mut self,
        io_handle: OTF2_IoHandleRef,
        mode: OTF2_IoAccessMode,
        status_flags: OTF2_IoStatusFlag,
    ) -> OTF2_CallbackCode {
        eprintln!(
            "visit_io_pre_created_handle_state: io_handle: {}, mode: {}, status_flags: {}",
            io_handle, mode, status_flags
        );
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    fn visit_unknown(&mut self) -> OTF2_CallbackCode {
        eprintln!("visit_unknown");
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }
}
