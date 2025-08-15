#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// These constants added manually because bindgen doesn't like to parse the '#define' statements
// for them

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const OTF2_UNDEFINED_UINT8: u8 = !0u8;
pub const OTF2_UNDEFINED_INT8: i8 = !(OTF2_UNDEFINED_UINT8 >> 1) as i8;
pub const OTF2_UNDEFINED_UINT16: u16 = !0u16;
pub const OTF2_UNDEFINED_INT16: i16 = !(OTF2_UNDEFINED_UINT16 >> 1) as i16;
pub const OTF2_UNDEFINED_UINT32: u32 = !0u32;
pub const OTF2_UNDEFINED_INT32: i32 = !(OTF2_UNDEFINED_UINT32 >> 1) as i32;
pub const OTF2_UNDEFINED_UINT64: u64 = !0u64;
pub const OTF2_UNDEFINED_INT64: i64 = !(OTF2_UNDEFINED_UINT64 >> 1) as i64;
pub const OTF2_UNDEFINED_TYPE: u8 = OTF2_UNDEFINED_UINT8;
pub const OTF2_UNDEFINED_TIMESTAMP: u64 = OTF2_UNDEFINED_UINT64;
pub const OTF2_UNDEFINED_IO_PARADIGM: OTF2_IoParadigmRef = OTF2_IoParadigmRef(OTF2_UNDEFINED_UINT8);
pub const OTF2_UNDEFINED_STRING: OTF2_StringRef = OTF2_StringRef(OTF2_UNDEFINED_UINT32);
pub const OTF2_UNDEFINED_ATTRIBUTE: OTF2_AttributeRef = OTF2_AttributeRef(OTF2_UNDEFINED_UINT32);
pub const OTF2_UNDEFINED_SYSTEM_TREE_NODE: OTF2_SystemTreeNodeRef =
    OTF2_SystemTreeNodeRef(OTF2_UNDEFINED_UINT32);
pub const OTF2_UNDEFINED_LOCATION_GROUP: OTF2_LocationGroupRef =
    OTF2_LocationGroupRef(OTF2_UNDEFINED_UINT32);
pub const OTF2_UNDEFINED_LOCATION: OTF2_LocationRef = OTF2_LocationRef(OTF2_UNDEFINED_UINT64);
pub const OTF2_UNDEFINED_REGION: OTF2_RegionRef = OTF2_RegionRef(OTF2_UNDEFINED_UINT32);
pub const OTF2_UNDEFINED_CALLSITE: OTF2_CallsiteRef = OTF2_CallsiteRef(OTF2_UNDEFINED_UINT32);
pub const OTF2_UNDEFINED_CALLPATH: OTF2_CallpathRef = OTF2_CallpathRef(OTF2_UNDEFINED_UINT32);
pub const OTF2_UNDEFINED_GROUP: OTF2_GroupRef = OTF2_GroupRef(OTF2_UNDEFINED_UINT32);
pub const OTF2_UNDEFINED_METRIC_MEMBER: OTF2_MetricMemberRef =
    OTF2_MetricMemberRef(OTF2_UNDEFINED_UINT32);
pub const OTF2_UNDEFINED_METRIC: OTF2_MetricRef = OTF2_MetricRef(OTF2_UNDEFINED_UINT32);
pub const OTF2_UNDEFINED_COMM: OTF2_CommRef = OTF2_CommRef(OTF2_UNDEFINED_UINT32);
pub const OTF2_UNDEFINED_PARAMETER: OTF2_ParameterRef = OTF2_ParameterRef(OTF2_UNDEFINED_UINT32);
pub const OTF2_UNDEFINED_RMA_WIN: OTF2_RmaWinRef = OTF2_RmaWinRef(OTF2_UNDEFINED_UINT32);
pub const OTF2_UNDEFINED_CART_DIMENSION: OTF2_CartDimensionRef =
    OTF2_CartDimensionRef(OTF2_UNDEFINED_UINT32);
pub const OTF2_UNDEFINED_CART_TOPOLOGY: OTF2_CartTopologyRef =
    OTF2_CartTopologyRef(OTF2_UNDEFINED_UINT32);
pub const OTF2_UNDEFINED_SOURCE_CODE_LOCATION: OTF2_SourceCodeLocationRef =
    OTF2_SourceCodeLocationRef(OTF2_UNDEFINED_UINT32);
pub const OTF2_UNDEFINED_CALLING_CONTEXT: OTF2_CallingContextRef =
    OTF2_CallingContextRef(OTF2_UNDEFINED_UINT32);
pub const OTF2_UNDEFINED_INTERRUPT_GENERATOR: OTF2_InterruptGeneratorRef =
    OTF2_InterruptGeneratorRef(OTF2_UNDEFINED_UINT32);
pub const OTF2_UNDEFINED_IO_FILE: OTF2_IoFileRef = OTF2_IoFileRef(OTF2_UNDEFINED_UINT32);
pub const OTF2_UNDEFINED_IO_HANDLE: OTF2_IoHandleRef = OTF2_IoHandleRef(OTF2_UNDEFINED_UINT32);

macro_rules! impl_display_for_newtype {
    ($newtype:ty) => {
        impl std::fmt::Display for $newtype {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({})", stringify!($newtype), self.0)
            }
        }
    };
}

impl_display_for_newtype!(OTF2_IoParadigmRef);
impl_display_for_newtype!(OTF2_StringRef);
impl_display_for_newtype!(OTF2_AttributeRef);
impl_display_for_newtype!(OTF2_SystemTreeNodeRef);
impl_display_for_newtype!(OTF2_LocationGroupRef);
impl_display_for_newtype!(OTF2_LocationRef);
impl_display_for_newtype!(OTF2_RegionRef);
impl_display_for_newtype!(OTF2_CallsiteRef);
impl_display_for_newtype!(OTF2_CallpathRef);
impl_display_for_newtype!(OTF2_GroupRef);
impl_display_for_newtype!(OTF2_MetricMemberRef);
impl_display_for_newtype!(OTF2_MetricRef);
impl_display_for_newtype!(OTF2_CommRef);
impl_display_for_newtype!(OTF2_ParameterRef);
impl_display_for_newtype!(OTF2_RmaWinRef);
impl_display_for_newtype!(OTF2_CartDimensionRef);
impl_display_for_newtype!(OTF2_CartTopologyRef);
impl_display_for_newtype!(OTF2_SourceCodeLocationRef);
impl_display_for_newtype!(OTF2_CallingContextRef);
impl_display_for_newtype!(OTF2_InterruptGeneratorRef);
impl_display_for_newtype!(OTF2_IoFileRef);
impl_display_for_newtype!(OTF2_IoHandleRef);
impl_display_for_newtype!(OTF2_Type);
impl_display_for_newtype!(OTF2_LocationType);
impl_display_for_newtype!(OTF2_LocationGroupType);
impl_display_for_newtype!(OTF2_GroupType);
impl_display_for_newtype!(OTF2_MetricType);
impl_display_for_newtype!(OTF2_ParameterType);

impl From<OTF2_Type> for OTF2_Type_enum {
    fn from(value: OTF2_Type) -> Self {
        use OTF2_Type_enum::*;
        match value {
            OTF2_Type(1) => OTF2_TYPE_UINT8,
            OTF2_Type(2) => OTF2_TYPE_UINT16,
            OTF2_Type(3) => OTF2_TYPE_UINT32,
            OTF2_Type(4) => OTF2_TYPE_UINT64,
            OTF2_Type(5) => OTF2_TYPE_INT8,
            OTF2_Type(6) => OTF2_TYPE_INT16,
            OTF2_Type(7) => OTF2_TYPE_INT32,
            OTF2_Type(8) => OTF2_TYPE_INT64,
            OTF2_Type(9) => OTF2_TYPE_FLOAT,
            OTF2_Type(10) => OTF2_TYPE_DOUBLE,
            OTF2_Type(11) => OTF2_TYPE_STRING,
            OTF2_Type(12) => OTF2_TYPE_ATTRIBUTE,
            OTF2_Type(13) => OTF2_TYPE_LOCATION,
            OTF2_Type(14) => OTF2_TYPE_REGION,
            OTF2_Type(15) => OTF2_TYPE_GROUP,
            OTF2_Type(16) => OTF2_TYPE_METRIC,
            OTF2_Type(17) => OTF2_TYPE_COMM,
            OTF2_Type(18) => OTF2_TYPE_PARAMETER,
            OTF2_Type(19) => OTF2_TYPE_RMA_WIN,
            OTF2_Type(20) => OTF2_TYPE_SOURCE_CODE_LOCATION,
            OTF2_Type(21) => OTF2_TYPE_CALLING_CONTEXT,
            OTF2_Type(22) => OTF2_TYPE_INTERRUPT_GENERATOR,
            OTF2_Type(23) => OTF2_TYPE_IO_FILE,
            OTF2_Type(24) => OTF2_TYPE_IO_HANDLE,
            OTF2_Type(25) => OTF2_TYPE_LOCATION_GROUP,
            OTF2_Type(_) => OTF2_TYPE_NONE,
        }
    }
}

impl OTF2_Type {
    pub fn to_enum(self) -> OTF2_Type_enum {
        self.into()
    }
}

impl OTF2_AttributeValue {
    pub fn display(&self, t: OTF2_Type) -> String {
        let t: OTF2_Type_enum = t.into();
        unsafe {
            match t {
                OTF2_Type_enum::OTF2_TYPE_NONE => format!("OTF2_TYPE_NONE({})", self.0.uint8),
                OTF2_Type_enum::OTF2_TYPE_UINT8 => format!("OTF2_TYPE_UINT8({})", self.0.uint8),
                OTF2_Type_enum::OTF2_TYPE_UINT16 => format!("OTF2_TYPE_UINT16({})", self.0.uint8),
                OTF2_Type_enum::OTF2_TYPE_UINT32 => format!("OTF2_TYPE_UINT32({})", self.0.uint8),
                OTF2_Type_enum::OTF2_TYPE_UINT64 => format!("OTF2_TYPE_UINT64({})", self.0.uint8),
                OTF2_Type_enum::OTF2_TYPE_INT8 => format!("OTF2_TYPE_INT8({})", self.0.uint8),
                OTF2_Type_enum::OTF2_TYPE_INT16 => format!("OTF2_TYPE_INT16({})", self.0.uint8),
                OTF2_Type_enum::OTF2_TYPE_INT32 => format!("OTF2_TYPE_INT32({})", self.0.uint8),
                OTF2_Type_enum::OTF2_TYPE_INT64 => format!("OTF2_TYPE_INT64({})", self.0.uint8),
                OTF2_Type_enum::OTF2_TYPE_FLOAT => format!("OTF2_TYPE_FLOAT({})", self.0.uint8),
                OTF2_Type_enum::OTF2_TYPE_DOUBLE => format!("OTF2_TYPE_DOUBLE({})", self.0.uint8),
                OTF2_Type_enum::OTF2_TYPE_STRING => format!("OTF2_TYPE_STRING({})", self.0.uint8),
                OTF2_Type_enum::OTF2_TYPE_ATTRIBUTE => {
                    format!("OTF2_TYPE_ATTRIBUTE({})", self.0.uint8)
                }
                OTF2_Type_enum::OTF2_TYPE_LOCATION => {
                    format!("OTF2_TYPE_LOCATION({})", self.0.uint8)
                }
                OTF2_Type_enum::OTF2_TYPE_REGION => format!("OTF2_TYPE_REGION({})", self.0.uint8),
                OTF2_Type_enum::OTF2_TYPE_GROUP => format!("OTF2_TYPE_GROUP({})", self.0.uint8),
                OTF2_Type_enum::OTF2_TYPE_METRIC => format!("OTF2_TYPE_METRIC({})", self.0.uint8),
                OTF2_Type_enum::OTF2_TYPE_COMM => format!("OTF2_TYPE_COMM({})", self.0.uint8),
                OTF2_Type_enum::OTF2_TYPE_PARAMETER => {
                    format!("OTF2_TYPE_PARAMETER({})", self.0.uint8)
                }
                OTF2_Type_enum::OTF2_TYPE_RMA_WIN => format!("OTF2_TYPE_RMA_WIN({})", self.0.uint8),
                OTF2_Type_enum::OTF2_TYPE_SOURCE_CODE_LOCATION => {
                    format!("OTF2_TYPE_SOURCE_CODE_LOCATION({})", self.0.uint8)
                }
                OTF2_Type_enum::OTF2_TYPE_CALLING_CONTEXT => {
                    format!("OTF2_TYPE_CALLING_CONTEXT({})", self.0.uint8)
                }
                OTF2_Type_enum::OTF2_TYPE_INTERRUPT_GENERATOR => {
                    format!("OTF2_TYPE_INTERRUPT_GENERATOR({})", self.0.uint8)
                }
                OTF2_Type_enum::OTF2_TYPE_IO_FILE => format!("OTF2_TYPE_IO_FILE({})", self.0.uint8),
                OTF2_Type_enum::OTF2_TYPE_IO_HANDLE => {
                    format!("OTF2_TYPE_IO_HANDLE({})", self.0.uint8)
                }
                OTF2_Type_enum::OTF2_TYPE_LOCATION_GROUP => {
                    format!("OTF2_TYPE_LOCATION_GROUP({})", self.0.uint8)
                }
            }
        }
    }
}

