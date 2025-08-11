use crate::error::Status;
use crate::internal::*;
use std::ffi::CStr;

pub trait DefinitionVisitor {
    fn visit_string(&mut self, defn: OTF2_StringRef, value: &CStr) -> OTF2_CallbackCode;
    fn visit_location(
        &mut self,
        defn: OTF2_LocationRef,
        name: OTF2_StringRef,
        location_type: OTF2_LocationType,
        num_events: u64,
        location_group: OTF2_LocationGroupRef,
    ) -> OTF2_CallbackCode;

    // Clock and paradigm definitions
    fn visit_clock_properties(
        &mut self,
        timer_resolution: u64,
        global_offset: u64,
        trace_length: u64,
        realtime_timestamp: u64,
    ) -> OTF2_CallbackCode;
    fn visit_paradigm(
        &mut self,
        paradigm: OTF2_Paradigm,
        name: OTF2_StringRef,
        paradigm_class: OTF2_ParadigmClass,
    ) -> OTF2_CallbackCode;
    fn visit_paradigm_property(
        &mut self,
        paradigm: OTF2_Paradigm,
        property: OTF2_ParadigmProperty,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode;
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
    ) -> OTF2_CallbackCode;

    // Definition attributes
    fn visit_attribute(
        &mut self,
        defn: OTF2_AttributeRef,
        name: OTF2_StringRef,
        description: OTF2_StringRef,
        type_: OTF2_Type,
    ) -> OTF2_CallbackCode;

    // System tree definitions
    fn visit_system_tree_node(
        &mut self,
        defn: OTF2_SystemTreeNodeRef,
        name: OTF2_StringRef,
        class_name: OTF2_StringRef,
        parent: Option<OTF2_SystemTreeNodeRef>,
    ) -> OTF2_CallbackCode;
    fn visit_system_tree_node_property(
        &mut self,
        system_tree_node: OTF2_SystemTreeNodeRef,
        name: OTF2_StringRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode;
    fn visit_system_tree_node_domain(
        &mut self,
        system_tree_node: OTF2_SystemTreeNodeRef,
        system_tree_domain: OTF2_SystemTreeDomain,
    ) -> OTF2_CallbackCode;

    // Location definitions
    fn visit_location_group(
        &mut self,
        defn: OTF2_LocationGroupRef,
        name: OTF2_StringRef,
        location_group_type: OTF2_LocationGroupType,
        system_tree_parent: OTF2_SystemTreeNodeRef,
        creating_location_group: Option<OTF2_LocationGroupRef>,
    ) -> OTF2_CallbackCode;
    fn visit_location_group_property(
        &mut self,
        location_group: OTF2_LocationGroupRef,
        name: OTF2_StringRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode;
    fn visit_location_property(
        &mut self,
        location: OTF2_LocationRef,
        name: OTF2_StringRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode;

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
    ) -> OTF2_CallbackCode;
    fn visit_callsite(
        &mut self,
        defn: OTF2_CallsiteRef,
        source_file: OTF2_StringRef,
        line_number: u32,
        entered_region: OTF2_RegionRef,
        left_region: OTF2_RegionRef,
    ) -> OTF2_CallbackCode;
    fn visit_callpath(
        &mut self,
        defn: OTF2_CallpathRef,
        parent: Option<OTF2_CallpathRef>,
        region: OTF2_RegionRef,
    ) -> OTF2_CallbackCode;
    fn visit_callpath_parameter(
        &mut self,
        callpath: OTF2_CallpathRef,
        parameter: OTF2_ParameterRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode;
    fn visit_source_code_location(
        &mut self,
        defn: OTF2_SourceCodeLocationRef,
        file: OTF2_StringRef,
        line_number: u32,
    ) -> OTF2_CallbackCode;
    fn visit_calling_context(
        &mut self,
        defn: OTF2_CallingContextRef,
        region: OTF2_RegionRef,
        source_code_location: OTF2_SourceCodeLocationRef,
        parent: Option<OTF2_CallingContextRef>,
    ) -> OTF2_CallbackCode;
    fn visit_calling_context_property(
        &mut self,
        calling_context: OTF2_CallingContextRef,
        name: OTF2_StringRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode;

    // Group definitions
    fn visit_group(
        &mut self,
        defn: OTF2_GroupRef,
        name: OTF2_StringRef,
        group_type: OTF2_GroupType,
        paradigm: OTF2_Paradigm,
        group_flags: OTF2_GroupFlag,
        members: &[u64],
    ) -> OTF2_CallbackCode;

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
    ) -> OTF2_CallbackCode;
    fn visit_metric_class(
        &mut self,
        defn: OTF2_MetricRef,
        metric_members: &[OTF2_MetricMemberRef],
        metric_occurrence: OTF2_MetricOccurrence,
        recorder_kind: OTF2_RecorderKind,
    ) -> OTF2_CallbackCode;
    fn visit_metric_instance(
        &mut self,
        defn: OTF2_MetricRef,
        metric_class: OTF2_MetricRef,
        recorder: OTF2_LocationRef,
        metric_scope: OTF2_MetricScope,
        scope: u64,
    ) -> OTF2_CallbackCode;
    fn visit_metric_class_recorder(
        &mut self,
        metric_class: OTF2_MetricRef,
        recorder: OTF2_LocationRef,
    ) -> OTF2_CallbackCode;

    // Communication definitions
    fn visit_comm(
        &mut self,
        defn: OTF2_CommRef,
        name: OTF2_StringRef,
        group: OTF2_GroupRef,
        parent: Option<OTF2_CommRef>,
    ) -> OTF2_CallbackCode;
    fn visit_inter_comm(
        &mut self,
        defn: OTF2_CommRef,
        name: OTF2_StringRef,
        group_a: OTF2_GroupRef,
        group_b: OTF2_GroupRef,
        common_communicator: Option<OTF2_CommRef>,
        flags: OTF2_CommFlag,
    ) -> OTF2_CallbackCode;
    fn visit_parameter(
        &mut self,
        defn: OTF2_ParameterRef,
        name: OTF2_StringRef,
        parameter_type: OTF2_ParameterType,
    ) -> OTF2_CallbackCode;
    fn visit_rma_win(
        &mut self,
        defn: OTF2_RmaWinRef,
        name: OTF2_StringRef,
        comm: OTF2_CommRef,
    ) -> OTF2_CallbackCode;

    // Cartesian topology definitions
    fn visit_cart_dimension(
        &mut self,
        defn: OTF2_CartDimensionRef,
        name: OTF2_StringRef,
        size: u32,
        periodic: OTF2_CartPeriodicity,
    ) -> OTF2_CallbackCode;
    fn visit_cart_topology(
        &mut self,
        defn: OTF2_CartTopologyRef,
        name: OTF2_StringRef,
        communicator: OTF2_CommRef,
        dimensions: &[OTF2_CartDimensionRef],
    ) -> OTF2_CallbackCode;
    fn visit_cart_coordinate(
        &mut self,
        topology: OTF2_CartTopologyRef,
        rank: u32,
        coordinates: &[u32],
    ) -> OTF2_CallbackCode;

    // Interrupt definitions
    fn visit_interrupt_generator(
        &mut self,
        defn: OTF2_InterruptGeneratorRef,
        name: OTF2_StringRef,
        interrupt_generator_mode: OTF2_InterruptGeneratorMode,
        base: OTF2_Base,
        exponent: i64,
        period: u64,
    ) -> OTF2_CallbackCode;

    // I/O definitions
    fn visit_io_file_property(
        &mut self,
        io_file: OTF2_IoFileRef,
        name: OTF2_StringRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode;
    fn visit_io_regular_file(
        &mut self,
        defn: OTF2_IoFileRef,
        name: OTF2_StringRef,
        scope: OTF2_SystemTreeNodeRef,
    ) -> OTF2_CallbackCode;
    fn visit_io_directory(
        &mut self,
        defn: OTF2_IoFileRef,
        name: OTF2_StringRef,
        scope: OTF2_SystemTreeNodeRef,
    ) -> OTF2_CallbackCode;
    fn visit_io_handle(
        &mut self,
        defn: OTF2_IoHandleRef,
        name: OTF2_StringRef,
        file: OTF2_IoFileRef,
        io_paradigm: OTF2_IoParadigmRef,
        io_handle_flags: OTF2_IoHandleFlag,
        comm: Option<OTF2_CommRef>,
        parent: Option<OTF2_IoHandleRef>,
    ) -> OTF2_CallbackCode;
    fn visit_io_pre_created_handle_state(
        &mut self,
        io_handle: OTF2_IoHandleRef,
        mode: OTF2_IoAccessMode,
        status_flags: OTF2_IoStatusFlag,
    ) -> OTF2_CallbackCode;

    // Unknown definition fallback
    fn visit_unknown(&mut self) -> OTF2_CallbackCode;
}

pub struct DefinitionVisitorMultiplexer {
    visitors: Vec<Box<dyn DefinitionVisitor>>,
}

impl DefinitionVisitorMultiplexer {
    pub fn new() -> Self {
        Self {
            visitors: Vec::new(),
        }
    }

    pub fn add_visitor(&mut self, visitor: Box<dyn DefinitionVisitor>) {
        self.visitors.push(visitor);
    }

    pub fn set_global_def_reader_callbacks<U>(
        &self,
        global_callbacks: &mut OwnedExternHandle<OTF2_GlobalDefReaderCallbacks_struct, U>,
    ) -> Result<(), Status> {
        unsafe {
            let cbs = global_callbacks.as_raw_mut();

            // Basic definitions
            OTF2_GlobalDefReaderCallbacks_SetUnknownCallback(cbs, Some(read_unknown_def))?;
            OTF2_GlobalDefReaderCallbacks_SetStringCallback(cbs, Some(read_string_def))?;
            OTF2_GlobalDefReaderCallbacks_SetAttributeCallback(cbs, Some(read_attribute_def))?;

            // Clock and paradigm definitions
            OTF2_GlobalDefReaderCallbacks_SetClockPropertiesCallback(
                cbs,
                Some(read_clock_properties_def),
            )?;
            OTF2_GlobalDefReaderCallbacks_SetParadigmCallback(cbs, Some(read_paradigm_def))?;
            OTF2_GlobalDefReaderCallbacks_SetParadigmPropertyCallback(
                cbs,
                Some(read_paradigm_property_def),
            )?;
            OTF2_GlobalDefReaderCallbacks_SetIoParadigmCallback(cbs, Some(read_io_paradigm_def))?;

            // System tree definitions
            OTF2_GlobalDefReaderCallbacks_SetSystemTreeNodeCallback(
                cbs,
                Some(read_system_tree_node_def),
            )?;
            OTF2_GlobalDefReaderCallbacks_SetSystemTreeNodePropertyCallback(
                cbs,
                Some(read_system_tree_node_property_def),
            )?;
            OTF2_GlobalDefReaderCallbacks_SetSystemTreeNodeDomainCallback(
                cbs,
                Some(read_system_tree_node_domain_def),
            )?;

            // Location definitions
            OTF2_GlobalDefReaderCallbacks_SetLocationGroupCallback(
                cbs,
                Some(read_location_group_def),
            )?;
            OTF2_GlobalDefReaderCallbacks_SetLocationCallback(cbs, Some(read_location_def))?;
            OTF2_GlobalDefReaderCallbacks_SetLocationGroupPropertyCallback(
                cbs,
                Some(read_location_group_property_def),
            )?;
            OTF2_GlobalDefReaderCallbacks_SetLocationPropertyCallback(
                cbs,
                Some(read_location_property_def),
            )?;

            // Region and callpath definitions
            OTF2_GlobalDefReaderCallbacks_SetRegionCallback(cbs, Some(read_region_def))?;
            OTF2_GlobalDefReaderCallbacks_SetCallsiteCallback(cbs, Some(read_callsite_def))?;
            OTF2_GlobalDefReaderCallbacks_SetCallpathCallback(cbs, Some(read_callpath_def))?;
            OTF2_GlobalDefReaderCallbacks_SetCallpathParameterCallback(
                cbs,
                Some(read_callpath_parameter_def),
            )?;
            OTF2_GlobalDefReaderCallbacks_SetSourceCodeLocationCallback(
                cbs,
                Some(read_source_code_location_def),
            )?;
            OTF2_GlobalDefReaderCallbacks_SetCallingContextCallback(
                cbs,
                Some(read_calling_context_def),
            )?;
            OTF2_GlobalDefReaderCallbacks_SetCallingContextPropertyCallback(
                cbs,
                Some(read_calling_context_property_def),
            )?;

            // Group definitions
            OTF2_GlobalDefReaderCallbacks_SetGroupCallback(cbs, Some(read_group_def))?;

            // Metric definitions
            OTF2_GlobalDefReaderCallbacks_SetMetricMemberCallback(
                cbs,
                Some(read_metric_member_def),
            )?;
            OTF2_GlobalDefReaderCallbacks_SetMetricClassCallback(cbs, Some(read_metric_class_def))?;
            OTF2_GlobalDefReaderCallbacks_SetMetricInstanceCallback(
                cbs,
                Some(read_metric_instance_def),
            )?;
            OTF2_GlobalDefReaderCallbacks_SetMetricClassRecorderCallback(
                cbs,
                Some(read_metric_class_recorder_def),
            )?;

            // Communication definitions
            OTF2_GlobalDefReaderCallbacks_SetCommCallback(cbs, Some(read_comm_def))?;
            OTF2_GlobalDefReaderCallbacks_SetInterCommCallback(cbs, Some(read_inter_comm_def))?;
            OTF2_GlobalDefReaderCallbacks_SetParameterCallback(cbs, Some(read_parameter_def))?;
            OTF2_GlobalDefReaderCallbacks_SetRmaWinCallback(cbs, Some(read_rma_win_def))?;

            // Cartesian topology definitions
            OTF2_GlobalDefReaderCallbacks_SetCartDimensionCallback(
                cbs,
                Some(read_cart_dimension_def),
            )?;
            OTF2_GlobalDefReaderCallbacks_SetCartTopologyCallback(
                cbs,
                Some(read_cart_topology_def),
            )?;
            OTF2_GlobalDefReaderCallbacks_SetCartCoordinateCallback(
                cbs,
                Some(read_cart_coordinate_def),
            )?;

            // Interrupt definitions
            OTF2_GlobalDefReaderCallbacks_SetInterruptGeneratorCallback(
                cbs,
                Some(read_interrupt_generator_def),
            )?;

            // I/O definitions
            OTF2_GlobalDefReaderCallbacks_SetIoFilePropertyCallback(
                cbs,
                Some(read_io_file_property_def),
            )?;
            OTF2_GlobalDefReaderCallbacks_SetIoRegularFileCallback(
                cbs,
                Some(read_io_regular_file_def),
            )?;
            OTF2_GlobalDefReaderCallbacks_SetIoDirectoryCallback(cbs, Some(read_io_directory_def))?;
            OTF2_GlobalDefReaderCallbacks_SetIoHandleCallback(cbs, Some(read_io_handle_def))?;
            OTF2_GlobalDefReaderCallbacks_SetIoPreCreatedHandleStateCallback(
                cbs,
                Some(read_io_pre_created_handle_state_def),
            )?;
        }
        Ok(())
    }
}

extern "C" fn read_string_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_StringRef,
    value: *const ::std::os::raw::c_char,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    let value = unsafe { std::ffi::CStr::from_ptr(value) };
    eprintln!(
        "visit_string: defn: {}, value: {}",
        defn,
        value.to_string_lossy()
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_string(defn, value);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_location_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_LocationRef,
    name: OTF2_StringRef,
    location_type: OTF2_LocationType,
    num_events: u64,
    location_group: OTF2_LocationGroupRef,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_location: defn: {}, name: {}, location_type: {}, num_events: {}, location_group: {}",
        defn, name, location_type, num_events, location_group
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_location(defn, name, location_type, num_events, location_group);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_unknown_def(user_data: *mut ::std::os::raw::c_void) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!("visit_unknown");
    for visitor in &mut this.visitors {
        let code = visitor.visit_unknown();
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_clock_properties_def(
    user_data: *mut ::std::os::raw::c_void,
    timer_resolution: u64,
    global_offset: u64,
    trace_length: u64,
    realtime_timestamp: u64,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_clock_properties: timer_resolution: {}, global_offset: {}, trace_length: {}, realtime_timestamp: {}",
        timer_resolution, global_offset, trace_length, realtime_timestamp
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_clock_properties(
            timer_resolution,
            global_offset,
            trace_length,
            realtime_timestamp,
        );
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_paradigm_def(
    user_data: *mut ::std::os::raw::c_void,
    paradigm: OTF2_Paradigm,
    name: OTF2_StringRef,
    paradigm_class: OTF2_ParadigmClass,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_paradigm: paradigm: {}, name: {}, paradigm_class: {}",
        paradigm, name, paradigm_class
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_paradigm(paradigm, name, paradigm_class);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_paradigm_property_def(
    user_data: *mut ::std::os::raw::c_void,
    paradigm: OTF2_Paradigm,
    property: OTF2_ParadigmProperty,
    type_: OTF2_Type,
    value: OTF2_AttributeValue,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_paradigm_property: paradigm: {}, property: {}, type: {}, value: <attr_value>",
        paradigm, property, type_
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_paradigm_property(paradigm, property, type_, value);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_io_paradigm_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_IoParadigmRef,
    identification: OTF2_StringRef,
    name: OTF2_StringRef,
    io_paradigm_class: OTF2_IoParadigmClass,
    io_paradigm_flags: OTF2_IoParadigmFlag,
    number_of_properties: u8,
    properties: *const OTF2_IoParadigmProperty,
    types: *const OTF2_Type,
    values: *const OTF2_AttributeValue,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    let properties_slice =
        unsafe { std::slice::from_raw_parts(properties, number_of_properties as usize) };
    let types_slice = unsafe { std::slice::from_raw_parts(types, number_of_properties as usize) };
    let values_slice = unsafe { std::slice::from_raw_parts(values, number_of_properties as usize) };
    eprintln!(
        "visit_io_paradigm: defn: {}, identification: {}, name: {}, io_paradigm_class: {}, io_paradigm_flags: {}, properties: {} items",
        defn, identification, name, io_paradigm_class, io_paradigm_flags, number_of_properties
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_io_paradigm(
            defn,
            identification,
            name,
            io_paradigm_class,
            io_paradigm_flags,
            properties_slice,
            types_slice,
            values_slice,
        );
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_attribute_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_AttributeRef,
    name: OTF2_StringRef,
    description: OTF2_StringRef,
    type_: OTF2_Type,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_attribute: defn: {}, name: {}, description: {}, type: {}",
        defn, name, description, type_
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_attribute(defn, name, description, type_);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_system_tree_node_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_SystemTreeNodeRef,
    name: OTF2_StringRef,
    class_name: OTF2_StringRef,
    parent: OTF2_SystemTreeNodeRef,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    let parent_opt = if parent == u32::MAX {
        None
    } else {
        Some(parent)
    };
    eprintln!(
        "visit_system_tree_node: defn: {}, name: {}, class_name: {}, parent: {:?}",
        defn, name, class_name, parent_opt
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_system_tree_node(defn, name, class_name, parent_opt);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_system_tree_node_property_def(
    user_data: *mut ::std::os::raw::c_void,
    system_tree_node: OTF2_SystemTreeNodeRef,
    name: OTF2_StringRef,
    type_: OTF2_Type,
    value: OTF2_AttributeValue,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_system_tree_node_property: system_tree_node: {}, name: {}, type: {}, value: <attr_value>",
        system_tree_node, name, type_
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_system_tree_node_property(system_tree_node, name, type_, value);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_system_tree_node_domain_def(
    user_data: *mut ::std::os::raw::c_void,
    system_tree_node: OTF2_SystemTreeNodeRef,
    system_tree_domain: OTF2_SystemTreeDomain,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_system_tree_node_domain: system_tree_node: {}, system_tree_domain: {}",
        system_tree_node, system_tree_domain
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_system_tree_node_domain(system_tree_node, system_tree_domain);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_location_group_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_LocationGroupRef,
    name: OTF2_StringRef,
    location_group_type: OTF2_LocationGroupType,
    system_tree_parent: OTF2_SystemTreeNodeRef,
    creating_location_group: OTF2_LocationGroupRef,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    let creating_location_group_opt = if creating_location_group == u32::MAX {
        None
    } else {
        Some(creating_location_group)
    };
    eprintln!(
        "visit_location_group: defn: {}, name: {}, location_group_type: {}, system_tree_parent: {}, creating_location_group: {:?}",
        defn, name, location_group_type, system_tree_parent, creating_location_group_opt
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_location_group(
            defn,
            name,
            location_group_type,
            system_tree_parent,
            creating_location_group_opt,
        );
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_location_group_property_def(
    user_data: *mut ::std::os::raw::c_void,
    location_group: OTF2_LocationGroupRef,
    name: OTF2_StringRef,
    type_: OTF2_Type,
    value: OTF2_AttributeValue,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_location_group_property: location_group: {}, name: {}, type: {}, value: <attr_value>",
        location_group, name, type_
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_location_group_property(location_group, name, type_, value);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_location_property_def(
    user_data: *mut ::std::os::raw::c_void,
    location: OTF2_LocationRef,
    name: OTF2_StringRef,
    type_: OTF2_Type,
    value: OTF2_AttributeValue,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_location_property: location: {}, name: {}, type: {}, value: <attr_value>",
        location, name, type_
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_location_property(location, name, type_, value);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_region_def(
    user_data: *mut ::std::os::raw::c_void,
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
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_region: defn: {}, name: {}, canonical_name: {}, description: {}, region_role: {}, paradigm: {}, region_flags: {}, source_file: {}, begin_line_number: {}, end_line_number: {}",
        defn,
        name,
        canonical_name,
        description,
        region_role,
        paradigm,
        region_flags,
        source_file,
        begin_line_number,
        end_line_number
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_region(
            defn,
            name,
            canonical_name,
            description,
            region_role,
            paradigm,
            region_flags,
            source_file,
            begin_line_number,
            end_line_number,
        );
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_callsite_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_CallsiteRef,
    source_file: OTF2_StringRef,
    line_number: u32,
    entered_region: OTF2_RegionRef,
    left_region: OTF2_RegionRef,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_callsite: defn: {}, source_file: {}, line_number: {}, entered_region: {}, left_region: {}",
        defn, source_file, line_number, entered_region, left_region
    );
    for visitor in &mut this.visitors {
        let code =
            visitor.visit_callsite(defn, source_file, line_number, entered_region, left_region);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_callpath_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_CallpathRef,
    parent: OTF2_CallpathRef,
    region: OTF2_RegionRef,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    let parent_opt = if parent == u32::MAX {
        None
    } else {
        Some(parent)
    };
    eprintln!(
        "visit_callpath: defn: {}, parent: {:?}, region: {}",
        defn, parent_opt, region
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_callpath(defn, parent_opt, region);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_callpath_parameter_def(
    user_data: *mut ::std::os::raw::c_void,
    callpath: OTF2_CallpathRef,
    parameter: OTF2_ParameterRef,
    type_: OTF2_Type,
    value: OTF2_AttributeValue,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_callpath_parameter: callpath: {}, parameter: {}, type: {}, value: <attr_value>",
        callpath, parameter, type_
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_callpath_parameter(callpath, parameter, type_, value);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_source_code_location_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_SourceCodeLocationRef,
    file: OTF2_StringRef,
    line_number: u32,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_source_code_location: defn: {}, file: {}, line_number: {}",
        defn, file, line_number
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_source_code_location(defn, file, line_number);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_calling_context_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_CallingContextRef,
    region: OTF2_RegionRef,
    source_code_location: OTF2_SourceCodeLocationRef,
    parent: OTF2_CallingContextRef,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    let parent_opt = if parent == u32::MAX {
        None
    } else {
        Some(parent)
    };
    eprintln!(
        "visit_calling_context: defn: {}, region: {}, source_code_location: {}, parent: {:?}",
        defn, region, source_code_location, parent_opt
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_calling_context(defn, region, source_code_location, parent_opt);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_calling_context_property_def(
    user_data: *mut ::std::os::raw::c_void,
    calling_context: OTF2_CallingContextRef,
    name: OTF2_StringRef,
    type_: OTF2_Type,
    value: OTF2_AttributeValue,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_calling_context_property: calling_context: {}, name: {}, type: {}, value: <attr_value>",
        calling_context, name, type_
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_calling_context_property(calling_context, name, type_, value);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_group_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_GroupRef,
    name: OTF2_StringRef,
    group_type: OTF2_GroupType,
    paradigm: OTF2_Paradigm,
    group_flags: OTF2_GroupFlag,
    number_of_members: u32,
    members: *const u64,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    let members_slice = unsafe { std::slice::from_raw_parts(members, number_of_members as usize) };
    eprintln!(
        "visit_group: defn: {}, name: {}, group_type: {}, paradigm: {}, group_flags: {}, members: {} items",
        defn, name, group_type, paradigm, group_flags, number_of_members
    );
    for visitor in &mut this.visitors {
        let code =
            visitor.visit_group(defn, name, group_type, paradigm, group_flags, members_slice);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_metric_member_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_MetricMemberRef,
    name: OTF2_StringRef,
    description: OTF2_StringRef,
    metric_type: OTF2_MetricType,
    metric_mode: OTF2_MetricMode,
    value_type: OTF2_Type,
    base: OTF2_Base,
    exponent: i64,
    unit: OTF2_StringRef,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_metric_member: defn: {}, name: {}, description: {}, metric_type: {}, metric_mode: {}, value_type: {}, base: {}, exponent: {}, unit: {}",
        defn, name, description, metric_type, metric_mode, value_type, base, exponent, unit
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_metric_member(
            defn,
            name,
            description,
            metric_type,
            metric_mode,
            value_type,
            base,
            exponent,
            unit,
        );
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_metric_class_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_MetricRef,
    number_of_metrics: u8,
    metric_members: *const OTF2_MetricMemberRef,
    metric_occurrence: OTF2_MetricOccurrence,
    recorder_kind: OTF2_RecorderKind,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    let metric_members_slice =
        unsafe { std::slice::from_raw_parts(metric_members, number_of_metrics as usize) };
    eprintln!(
        "visit_metric_class: defn: {}, metric_members: {} items, metric_occurrence: {}, recorder_kind: {}",
        defn, number_of_metrics, metric_occurrence, recorder_kind
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_metric_class(
            defn,
            metric_members_slice,
            metric_occurrence,
            recorder_kind,
        );
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_metric_instance_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_MetricRef,
    metric_class: OTF2_MetricRef,
    recorder: OTF2_LocationRef,
    metric_scope: OTF2_MetricScope,
    scope: u64,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_metric_instance: defn: {}, metric_class: {}, recorder: {}, metric_scope: {}, scope: {}",
        defn, metric_class, recorder, metric_scope, scope
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_metric_instance(defn, metric_class, recorder, metric_scope, scope);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_metric_class_recorder_def(
    user_data: *mut ::std::os::raw::c_void,
    metric_class: OTF2_MetricRef,
    recorder: OTF2_LocationRef,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_metric_class_recorder: metric_class: {}, recorder: {}",
        metric_class, recorder
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_metric_class_recorder(metric_class, recorder);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_comm_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_CommRef,
    name: OTF2_StringRef,
    group: OTF2_GroupRef,
    parent: OTF2_CommRef,
    flags: OTF2_CommFlag,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    let parent_opt = if parent == u32::MAX {
        None
    } else {
        Some(parent)
    };
    eprintln!(
        "visit_comm: defn: {}, name: {}, group: {}, parent: {:?}, flags: {}",
        defn, name, group, parent_opt, flags
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_comm(defn, name, group, parent_opt);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_inter_comm_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_CommRef,
    name: OTF2_StringRef,
    group_a: OTF2_GroupRef,
    group_b: OTF2_GroupRef,
    common_communicator: OTF2_CommRef,
    flags: OTF2_CommFlag,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    let common_communicator_opt = if common_communicator == u32::MAX {
        None
    } else {
        Some(common_communicator)
    };
    eprintln!(
        "visit_inter_comm: defn: {}, name: {}, group_a: {}, group_b: {}, common_communicator: {:?}, flags: {}",
        defn, name, group_a, group_b, common_communicator_opt, flags
    );
    for visitor in &mut this.visitors {
        let code =
            visitor.visit_inter_comm(defn, name, group_a, group_b, common_communicator_opt, flags);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_parameter_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_ParameterRef,
    name: OTF2_StringRef,
    parameter_type: OTF2_ParameterType,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_parameter: defn: {}, name: {}, parameter_type: {}",
        defn, name, parameter_type
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_parameter(defn, name, parameter_type);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_rma_win_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_RmaWinRef,
    name: OTF2_StringRef,
    comm: OTF2_CommRef,
    flags: OTF2_RmaWinFlag,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_rma_win: defn: {}, name: {}, comm: {}, flags: {}",
        defn, name, comm, flags
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_rma_win(defn, name, comm);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_cart_dimension_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_CartDimensionRef,
    name: OTF2_StringRef,
    size: u32,
    periodic: OTF2_CartPeriodicity,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_cart_dimension: defn: {}, name: {}, size: {}, periodic: {}",
        defn, name, size, periodic
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_cart_dimension(defn, name, size, periodic);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_cart_topology_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_CartTopologyRef,
    name: OTF2_StringRef,
    communicator: OTF2_CommRef,
    number_of_dimensions: u8,
    dimensions: *const OTF2_CartDimensionRef,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    let dimensions_slice =
        unsafe { std::slice::from_raw_parts(dimensions, number_of_dimensions as usize) };
    eprintln!(
        "visit_cart_topology: defn: {}, name: {}, communicator: {}, dimensions: {} items",
        defn, name, communicator, number_of_dimensions
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_cart_topology(defn, name, communicator, dimensions_slice);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_cart_coordinate_def(
    user_data: *mut ::std::os::raw::c_void,
    topology: OTF2_CartTopologyRef,
    rank: u32,
    number_of_coordinates: u8,
    coordinates: *const u32,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    let coordinates_slice =
        unsafe { std::slice::from_raw_parts(coordinates, number_of_coordinates as usize) };
    eprintln!(
        "visit_cart_coordinate: topology: {}, rank: {}, coordinates: {} items",
        topology, rank, number_of_coordinates
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_cart_coordinate(topology, rank, coordinates_slice);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_interrupt_generator_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_InterruptGeneratorRef,
    name: OTF2_StringRef,
    interrupt_generator_mode: OTF2_InterruptGeneratorMode,
    base: OTF2_Base,
    exponent: i64,
    period: u64,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_interrupt_generator: defn: {}, name: {}, interrupt_generator_mode: {}, base: {}, exponent: {}, period: {}",
        defn, name, interrupt_generator_mode, base, exponent, period
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_interrupt_generator(
            defn,
            name,
            interrupt_generator_mode,
            base,
            exponent,
            period,
        );
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_io_file_property_def(
    user_data: *mut ::std::os::raw::c_void,
    io_file: OTF2_IoFileRef,
    name: OTF2_StringRef,
    type_: OTF2_Type,
    value: OTF2_AttributeValue,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_io_file_property: io_file: {}, name: {}, type: {}, value: <attr_value>",
        io_file, name, type_
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_io_file_property(io_file, name, type_, value);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_io_regular_file_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_IoFileRef,
    name: OTF2_StringRef,
    scope: OTF2_SystemTreeNodeRef,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_io_regular_file: defn: {}, name: {}, scope: {}",
        defn, name, scope
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_io_regular_file(defn, name, scope);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_io_directory_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_IoFileRef,
    name: OTF2_StringRef,
    scope: OTF2_SystemTreeNodeRef,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_io_directory: defn: {}, name: {}, scope: {}",
        defn, name, scope
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_io_directory(defn, name, scope);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_io_handle_def(
    user_data: *mut ::std::os::raw::c_void,
    defn: OTF2_IoHandleRef,
    name: OTF2_StringRef,
    file: OTF2_IoFileRef,
    io_paradigm: OTF2_IoParadigmRef,
    io_handle_flags: OTF2_IoHandleFlag,
    comm: OTF2_CommRef,
    parent: OTF2_IoHandleRef,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    let comm_opt = if comm == u32::MAX { None } else { Some(comm) };
    let parent_opt = if parent == u32::MAX {
        None
    } else {
        Some(parent)
    };
    eprintln!(
        "visit_io_handle: defn: {}, name: {}, file: {}, io_paradigm: {}, io_handle_flags: {}, comm: {:?}, parent: {:?}",
        defn, name, file, io_paradigm, io_handle_flags, comm_opt, parent_opt
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_io_handle(
            defn,
            name,
            file,
            io_paradigm,
            io_handle_flags,
            comm_opt,
            parent_opt,
        );
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}

extern "C" fn read_io_pre_created_handle_state_def(
    user_data: *mut ::std::os::raw::c_void,
    io_handle: OTF2_IoHandleRef,
    mode: OTF2_IoAccessMode,
    status_flags: OTF2_IoStatusFlag,
) -> OTF2_CallbackCode_enum {
    let this: &mut DefinitionVisitorMultiplexer = unsafe { &mut *(user_data as *mut _) };
    eprintln!(
        "visit_io_pre_created_handle_state: io_handle: {}, mode: {}, status_flags: {}",
        io_handle, mode, status_flags
    );
    for visitor in &mut this.visitors {
        let code = visitor.visit_io_pre_created_handle_state(io_handle, mode, status_flags);
        if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
            return code;
        }
    }
    OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
}
