#![allow(unused_variables)]

use crate::internal::*;
use std::ffi::CStr;

pub trait DefinitionVisitor: std::fmt::Debug {
    fn visit_string(&mut self, defn: OTF2_StringRef, value: &CStr) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_location(
        &mut self,
        defn: OTF2_LocationRef,
        name: OTF2_StringRef,
        location_type: OTF2_LocationType,
        num_events: u64,
        location_group: OTF2_LocationGroupRef,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }

    // Clock and paradigm definitions
    fn visit_clock_properties(
        &mut self,
        timer_resolution: u64,
        global_offset: u64,
        trace_length: u64,
        realtime_timestamp: u64,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_paradigm(
        &mut self,
        paradigm: OTF2_Paradigm,
        name: OTF2_StringRef,
        paradigm_class: OTF2_ParadigmClass,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_paradigm_property(
        &mut self,
        paradigm: OTF2_Paradigm,
        property: OTF2_ParadigmProperty,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
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
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }

    // Definition attributes
    fn visit_attribute(
        &mut self,
        defn: OTF2_AttributeRef,
        name: OTF2_StringRef,
        description: OTF2_StringRef,
        type_: OTF2_Type,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }

    // System tree definitions
    fn visit_system_tree_node(
        &mut self,
        defn: OTF2_SystemTreeNodeRef,
        name: OTF2_StringRef,
        class_name: OTF2_StringRef,
        parent: Option<OTF2_SystemTreeNodeRef>,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_system_tree_node_property(
        &mut self,
        system_tree_node: OTF2_SystemTreeNodeRef,
        name: OTF2_StringRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_system_tree_node_domain(
        &mut self,
        system_tree_node: OTF2_SystemTreeNodeRef,
        system_tree_domain: OTF2_SystemTreeDomain,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }

    // Location definitions
    fn visit_location_group(
        &mut self,
        defn: OTF2_LocationGroupRef,
        name: OTF2_StringRef,
        location_group_type: OTF2_LocationGroupType,
        system_tree_parent: OTF2_SystemTreeNodeRef,
        creating_location_group: Option<OTF2_LocationGroupRef>,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_location_group_property(
        &mut self,
        location_group: OTF2_LocationGroupRef,
        name: OTF2_StringRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_location_property(
        &mut self,
        location: OTF2_LocationRef,
        name: OTF2_StringRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }

    // Region and callpath definitions
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
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_callsite(
        &mut self,
        defn: OTF2_CallsiteRef,
        source_file: OTF2_StringRef,
        line_number: u32,
        entered_region: OTF2_RegionRef,
        left_region: OTF2_RegionRef,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_callpath(
        &mut self,
        defn: OTF2_CallpathRef,
        parent: Option<OTF2_CallpathRef>,
        region: OTF2_RegionRef,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_callpath_parameter(
        &mut self,
        callpath: OTF2_CallpathRef,
        parameter: OTF2_ParameterRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_source_code_location(
        &mut self,
        defn: OTF2_SourceCodeLocationRef,
        file: OTF2_StringRef,
        line_number: u32,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_calling_context(
        &mut self,
        defn: OTF2_CallingContextRef,
        region: OTF2_RegionRef,
        source_code_location: OTF2_SourceCodeLocationRef,
        parent: Option<OTF2_CallingContextRef>,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_calling_context_property(
        &mut self,
        calling_context: OTF2_CallingContextRef,
        name: OTF2_StringRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }

    // Group definitions
    fn visit_group(
        &mut self,
        defn: OTF2_GroupRef,
        name: OTF2_StringRef,
        group_type: OTF2_GroupType,
        paradigm: OTF2_Paradigm,
        group_flags: OTF2_GroupFlag,
        members: &[u64],
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }

    // Metric definitions
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
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_metric_class(
        &mut self,
        defn: OTF2_MetricRef,
        metric_members: &[OTF2_MetricMemberRef],
        metric_occurrence: OTF2_MetricOccurrence,
        recorder_kind: OTF2_RecorderKind,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_metric_instance(
        &mut self,
        defn: OTF2_MetricRef,
        metric_class: OTF2_MetricRef,
        recorder: OTF2_LocationRef,
        metric_scope: OTF2_MetricScope,
        scope: u64,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_metric_class_recorder(
        &mut self,
        metric_class: OTF2_MetricRef,
        recorder: OTF2_LocationRef,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }

    // Communication definitions
    fn visit_comm(
        &mut self,
        defn: OTF2_CommRef,
        name: OTF2_StringRef,
        group: OTF2_GroupRef,
        parent: Option<OTF2_CommRef>,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_inter_comm(
        &mut self,
        defn: OTF2_CommRef,
        name: OTF2_StringRef,
        group_a: OTF2_GroupRef,
        group_b: OTF2_GroupRef,
        common_communicator: Option<OTF2_CommRef>,
        flags: OTF2_CommFlag,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_parameter(
        &mut self,
        defn: OTF2_ParameterRef,
        name: OTF2_StringRef,
        parameter_type: OTF2_ParameterType,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_rma_win(
        &mut self,
        defn: OTF2_RmaWinRef,
        name: OTF2_StringRef,
        comm: OTF2_CommRef,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }

    // Cartesian topology definitions
    fn visit_cart_dimension(
        &mut self,
        defn: OTF2_CartDimensionRef,
        name: OTF2_StringRef,
        size: u32,
        periodic: OTF2_CartPeriodicity,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_cart_topology(
        &mut self,
        defn: OTF2_CartTopologyRef,
        name: OTF2_StringRef,
        communicator: OTF2_CommRef,
        dimensions: &[OTF2_CartDimensionRef],
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_cart_coordinate(
        &mut self,
        topology: OTF2_CartTopologyRef,
        rank: u32,
        coordinates: &[u32],
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }

    // Interrupt definitions
    fn visit_interrupt_generator(
        &mut self,
        defn: OTF2_InterruptGeneratorRef,
        name: OTF2_StringRef,
        interrupt_generator_mode: OTF2_InterruptGeneratorMode,
        base: OTF2_Base,
        exponent: i64,
        period: u64,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }

    // I/O definitions
    fn visit_io_file_property(
        &mut self,
        io_file: OTF2_IoFileRef,
        name: OTF2_StringRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_io_regular_file(
        &mut self,
        defn: OTF2_IoFileRef,
        name: OTF2_StringRef,
        scope: OTF2_SystemTreeNodeRef,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_io_directory(
        &mut self,
        defn: OTF2_IoFileRef,
        name: OTF2_StringRef,
        scope: OTF2_SystemTreeNodeRef,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_io_handle(
        &mut self,
        defn: OTF2_IoHandleRef,
        name: OTF2_StringRef,
        file: OTF2_IoFileRef,
        io_paradigm: OTF2_IoParadigmRef,
        io_handle_flags: OTF2_IoHandleFlag,
        comm: Option<OTF2_CommRef>,
        parent: Option<OTF2_IoHandleRef>,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
    fn visit_io_pre_created_handle_state(
        &mut self,
        io_handle: OTF2_IoHandleRef,
        mode: OTF2_IoAccessMode,
        status_flags: OTF2_IoStatusFlag,
    ) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }

    // Unknown definition fallback
    fn visit_unknown(&mut self) -> OTF2_CallbackCode { OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS }
}
