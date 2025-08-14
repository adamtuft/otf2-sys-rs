#![allow(unused_variables)]

use crate::error::Status;
use crate::internal::*;
use std::ffi::CStr;

use super::visitor::DefinitionVisitor;

/// Safe wrapper around OTF2_GlobalDefReaderCallbacks
/// 
/// Registers callbacks for reading global definitions in OTF2 traces. These callbacks expect a
/// vector of mutable references to `DefinitionVisitor` trait objects, each of which will be
/// notified of the definitions as they are read from the trace.
#[derive(Debug, derive_more::Deref, derive_more::DerefMut)]
pub struct GlobalDefReaderCallbacks(Handle<OTF2_GlobalDefReaderCallbacks_struct>);

impl core::ops::Drop for GlobalDefReaderCallbacks {
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { OTF2_GlobalDefReaderCallbacks_Delete(self.take()) };
        }
    }
}

impl GlobalDefReaderCallbacks {
    pub fn new() -> Status<Self> {
        let mut this = Self(Handle::from_raw(unsafe { OTF2_GlobalDefReaderCallbacks_New() })
            .expect("Failed to create GlobalDefReaderCallbacks: null pointer"));
        this.set_callbacks()?;
        Ok(this)
    }

    fn set_callbacks(&mut self) -> Status<()> {
        use visitor_callbacks::*;
        let cbs = self.as_mut_ptr();
        unsafe {
            OTF2_GlobalDefReaderCallbacks_SetUnknownCallback(cbs, Some(read_unknown_def))?;
            OTF2_GlobalDefReaderCallbacks_SetStringCallback(cbs, Some(read_string_def))?;
            OTF2_GlobalDefReaderCallbacks_SetAttributeCallback(cbs, Some(read_attribute_def))?;
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
            OTF2_GlobalDefReaderCallbacks_SetGroupCallback(cbs, Some(read_group_def))?;
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
            OTF2_GlobalDefReaderCallbacks_SetCommCallback(cbs, Some(read_comm_def))?;
            OTF2_GlobalDefReaderCallbacks_SetInterCommCallback(cbs, Some(read_inter_comm_def))?;
            OTF2_GlobalDefReaderCallbacks_SetParameterCallback(cbs, Some(read_parameter_def))?;
            OTF2_GlobalDefReaderCallbacks_SetRmaWinCallback(cbs, Some(read_rma_win_def))?;
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
            OTF2_GlobalDefReaderCallbacks_SetInterruptGeneratorCallback(
                cbs,
                Some(read_interrupt_generator_def),
            )?;
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

mod visitor_callbacks {
    use super::*;

    #[inline]
    fn as_visitors<'a, 'b>(data: *mut ::std::os::raw::c_void) -> &'a mut Vec<&'b mut dyn DefinitionVisitor> {
        unsafe { &mut *(data as *mut Vec<&mut dyn DefinitionVisitor>) }
    }

    pub extern "C" fn read_string_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_StringRef,
        value: *const ::std::os::raw::c_char,
    ) -> OTF2_CallbackCode_enum {
        let value = unsafe { CStr::from_ptr(value) };
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_string(defn, value);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_location_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_LocationRef,
        name: OTF2_StringRef,
        location_type: OTF2_LocationType,
        num_events: u64,
        location_group: OTF2_LocationGroupRef,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_location(defn, name, location_type, num_events, location_group);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_unknown_def(user_data: *mut ::std::os::raw::c_void) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_unknown();
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_clock_properties_def(
        user_data: *mut ::std::os::raw::c_void,
        timer_resolution: u64,
        global_offset: u64,
        trace_length: u64,
        realtime_timestamp: u64,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
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

    pub extern "C" fn read_paradigm_def(
        user_data: *mut ::std::os::raw::c_void,
        paradigm: OTF2_Paradigm,
        name: OTF2_StringRef,
        paradigm_class: OTF2_ParadigmClass,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_paradigm(paradigm, name, paradigm_class);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_paradigm_property_def(
        user_data: *mut ::std::os::raw::c_void,
        paradigm: OTF2_Paradigm,
        property: OTF2_ParadigmProperty,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_paradigm_property(paradigm, property, type_, value);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_io_paradigm_def(
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
        let properties_slice =
            unsafe { std::slice::from_raw_parts(properties, number_of_properties as usize) };
        let types_slice = unsafe { std::slice::from_raw_parts(types, number_of_properties as usize) };
        let values_slice = unsafe { std::slice::from_raw_parts(values, number_of_properties as usize) };
        for visitor in as_visitors(user_data) {
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

    pub extern "C" fn read_attribute_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_AttributeRef,
        name: OTF2_StringRef,
        description: OTF2_StringRef,
        type_: OTF2_Type,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_attribute(defn, name, description, type_);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_system_tree_node_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_SystemTreeNodeRef,
        name: OTF2_StringRef,
        class_name: OTF2_StringRef,
        parent: OTF2_SystemTreeNodeRef,
    ) -> OTF2_CallbackCode_enum {
        let parent_opt = if parent == OTF2_UNDEFINED_SYSTEM_TREE_NODE {
            None
        } else {
            Some(parent)
        };
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_system_tree_node(defn, name, class_name, parent_opt);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_system_tree_node_property_def(
        user_data: *mut ::std::os::raw::c_void,
        system_tree_node: OTF2_SystemTreeNodeRef,
        name: OTF2_StringRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_system_tree_node_property(system_tree_node, name, type_, value);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_system_tree_node_domain_def(
        user_data: *mut ::std::os::raw::c_void,
        system_tree_node: OTF2_SystemTreeNodeRef,
        system_tree_domain: OTF2_SystemTreeDomain,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_system_tree_node_domain(system_tree_node, system_tree_domain);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_location_group_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_LocationGroupRef,
        name: OTF2_StringRef,
        location_group_type: OTF2_LocationGroupType,
        system_tree_parent: OTF2_SystemTreeNodeRef,
        creating_location_group: OTF2_LocationGroupRef,
    ) -> OTF2_CallbackCode_enum {
        let creating_location_group_opt = if creating_location_group == OTF2_UNDEFINED_LOCATION_GROUP {
            None
        } else {
            Some(creating_location_group)
        };
        for visitor in as_visitors(user_data) {
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

    pub extern "C" fn read_location_group_property_def(
        user_data: *mut ::std::os::raw::c_void,
        location_group: OTF2_LocationGroupRef,
        name: OTF2_StringRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_location_group_property(location_group, name, type_, value);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_location_property_def(
        user_data: *mut ::std::os::raw::c_void,
        location: OTF2_LocationRef,
        name: OTF2_StringRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_location_property(location, name, type_, value);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_region_def(
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
        for visitor in as_visitors(user_data) {
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

    pub extern "C" fn read_callsite_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_CallsiteRef,
        source_file: OTF2_StringRef,
        line_number: u32,
        entered_region: OTF2_RegionRef,
        left_region: OTF2_RegionRef,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code =
                visitor.visit_callsite(defn, source_file, line_number, entered_region, left_region);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_callpath_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_CallpathRef,
        parent: OTF2_CallpathRef,
        region: OTF2_RegionRef,
    ) -> OTF2_CallbackCode_enum {
        let parent_opt = if parent == OTF2_UNDEFINED_CALLPATH {
            None
        } else {
            Some(parent)
        };
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_callpath(defn, parent_opt, region);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_callpath_parameter_def(
        user_data: *mut ::std::os::raw::c_void,
        callpath: OTF2_CallpathRef,
        parameter: OTF2_ParameterRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_callpath_parameter(callpath, parameter, type_, value);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_source_code_location_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_SourceCodeLocationRef,
        file: OTF2_StringRef,
        line_number: u32,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_source_code_location(defn, file, line_number);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_calling_context_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_CallingContextRef,
        region: OTF2_RegionRef,
        source_code_location: OTF2_SourceCodeLocationRef,
        parent: OTF2_CallingContextRef,
    ) -> OTF2_CallbackCode_enum {
        let parent_opt = if parent == OTF2_UNDEFINED_CALLING_CONTEXT {
            None
        } else {
            Some(parent)
        };
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_calling_context(defn, region, source_code_location, parent_opt);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_calling_context_property_def(
        user_data: *mut ::std::os::raw::c_void,
        calling_context: OTF2_CallingContextRef,
        name: OTF2_StringRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_calling_context_property(calling_context, name, type_, value);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_group_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_GroupRef,
        name: OTF2_StringRef,
        group_type: OTF2_GroupType,
        paradigm: OTF2_Paradigm,
        group_flags: OTF2_GroupFlag,
        number_of_members: u32,
        members: *const u64,
    ) -> OTF2_CallbackCode_enum {
        let members_slice = unsafe { std::slice::from_raw_parts(members, number_of_members as usize) };
        for visitor in as_visitors(user_data) {
            let code =
                visitor.visit_group(defn, name, group_type, paradigm, group_flags, members_slice);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_metric_member_def(
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
        for visitor in as_visitors(user_data) {
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

    pub extern "C" fn read_metric_class_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_MetricRef,
        number_of_metrics: u8,
        metric_members: *const OTF2_MetricMemberRef,
        metric_occurrence: OTF2_MetricOccurrence,
        recorder_kind: OTF2_RecorderKind,
    ) -> OTF2_CallbackCode_enum {
        let metric_members_slice =
            unsafe { std::slice::from_raw_parts(metric_members, number_of_metrics as usize) };
        for visitor in as_visitors(user_data) {
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

    pub extern "C" fn read_metric_instance_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_MetricRef,
        metric_class: OTF2_MetricRef,
        recorder: OTF2_LocationRef,
        metric_scope: OTF2_MetricScope,
        scope: u64,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_metric_instance(defn, metric_class, recorder, metric_scope, scope);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_metric_class_recorder_def(
        user_data: *mut ::std::os::raw::c_void,
        metric_class: OTF2_MetricRef,
        recorder: OTF2_LocationRef,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_metric_class_recorder(metric_class, recorder);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_comm_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_CommRef,
        name: OTF2_StringRef,
        group: OTF2_GroupRef,
        parent: OTF2_CommRef,
        flags: OTF2_CommFlag,
    ) -> OTF2_CallbackCode_enum {
        let parent_opt = if parent == OTF2_UNDEFINED_COMM {
            None
        } else {
            Some(parent)
        };
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_comm(defn, name, group, parent_opt);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_inter_comm_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_CommRef,
        name: OTF2_StringRef,
        group_a: OTF2_GroupRef,
        group_b: OTF2_GroupRef,
        common_communicator: OTF2_CommRef,
        flags: OTF2_CommFlag,
    ) -> OTF2_CallbackCode_enum {
        let common_communicator_opt = if common_communicator == OTF2_UNDEFINED_COMM {
            None
        } else {
            Some(common_communicator)
        };
        for visitor in as_visitors(user_data) {
            let code =
                visitor.visit_inter_comm(defn, name, group_a, group_b, common_communicator_opt, flags);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_parameter_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_ParameterRef,
        name: OTF2_StringRef,
        parameter_type: OTF2_ParameterType,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_parameter(defn, name, parameter_type);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_rma_win_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_RmaWinRef,
        name: OTF2_StringRef,
        comm: OTF2_CommRef,
        flags: OTF2_RmaWinFlag,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_rma_win(defn, name, comm);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_cart_dimension_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_CartDimensionRef,
        name: OTF2_StringRef,
        size: u32,
        periodic: OTF2_CartPeriodicity,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_cart_dimension(defn, name, size, periodic);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_cart_topology_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_CartTopologyRef,
        name: OTF2_StringRef,
        communicator: OTF2_CommRef,
        number_of_dimensions: u8,
        dimensions: *const OTF2_CartDimensionRef,
    ) -> OTF2_CallbackCode_enum {
        let dimensions_slice =
            unsafe { std::slice::from_raw_parts(dimensions, number_of_dimensions as usize) };
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_cart_topology(defn, name, communicator, dimensions_slice);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_cart_coordinate_def(
        user_data: *mut ::std::os::raw::c_void,
        topology: OTF2_CartTopologyRef,
        rank: u32,
        number_of_coordinates: u8,
        coordinates: *const u32,
    ) -> OTF2_CallbackCode_enum {
        let coordinates_slice =
            unsafe { std::slice::from_raw_parts(coordinates, number_of_coordinates as usize) };
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_cart_coordinate(topology, rank, coordinates_slice);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_interrupt_generator_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_InterruptGeneratorRef,
        name: OTF2_StringRef,
        interrupt_generator_mode: OTF2_InterruptGeneratorMode,
        base: OTF2_Base,
        exponent: i64,
        period: u64,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
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

    pub extern "C" fn read_io_file_property_def(
        user_data: *mut ::std::os::raw::c_void,
        io_file: OTF2_IoFileRef,
        name: OTF2_StringRef,
        type_: OTF2_Type,
        value: OTF2_AttributeValue,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_io_file_property(io_file, name, type_, value);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_io_regular_file_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_IoFileRef,
        name: OTF2_StringRef,
        scope: OTF2_SystemTreeNodeRef,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_io_regular_file(defn, name, scope);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_io_directory_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_IoFileRef,
        name: OTF2_StringRef,
        scope: OTF2_SystemTreeNodeRef,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_io_directory(defn, name, scope);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }

    pub extern "C" fn read_io_handle_def(
        user_data: *mut ::std::os::raw::c_void,
        defn: OTF2_IoHandleRef,
        name: OTF2_StringRef,
        file: OTF2_IoFileRef,
        io_paradigm: OTF2_IoParadigmRef,
        io_handle_flags: OTF2_IoHandleFlag,
        comm: OTF2_CommRef,
        parent: OTF2_IoHandleRef,
    ) -> OTF2_CallbackCode_enum {
        let comm_opt = if comm == OTF2_UNDEFINED_COMM {
            None
        } else {
            Some(comm)
        };
        let parent_opt = if parent == OTF2_UNDEFINED_IO_HANDLE {
            None
        } else {
            Some(parent)
        };
        for visitor in as_visitors(user_data) {
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

    pub extern "C" fn read_io_pre_created_handle_state_def(
        user_data: *mut ::std::os::raw::c_void,
        io_handle: OTF2_IoHandleRef,
        mode: OTF2_IoAccessMode,
        status_flags: OTF2_IoStatusFlag,
    ) -> OTF2_CallbackCode_enum {
        for visitor in as_visitors(user_data) {
            let code = visitor.visit_io_pre_created_handle_state(io_handle, mode, status_flags);
            if code != OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS {
                return code;
            }
        }
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }
}
