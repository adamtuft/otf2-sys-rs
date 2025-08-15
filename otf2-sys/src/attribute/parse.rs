use crate::internal::*;
use crate::error::{Status, StatusCode};
use std::ffi::{CString, CStr};
use std::ops::ControlFlow;

#[derive(Debug)]
pub struct AttributeIterator {
    index: u32,
    length: usize,
    handle: Handle<OTF2_AttributeList>,
}

impl AttributeIterator {
    pub(crate) fn new(handle: Handle<OTF2_AttributeList>) -> Self {
        Self { index: 0, length: handle.length() as usize, handle }
    }
}

impl std::iter::Iterator for AttributeIterator {
    type Item = (OTF2_AttributeRef, AttributeValue);

    fn next(&mut self) -> Option<Self::Item> {
        if (self.index as usize) >= self.length {
            None
        } else {
            let cur = self.index; 
            self.index += 1;
            // SAFETY: safe as long as the handle is valid and we are in bounds.
            Some(self.handle.get_attribute_by_index(cur).expect("Failed to get attribute"))
        }
    }
}

impl std::iter::IntoIterator for Handle<OTF2_AttributeList> {
    type Item = (OTF2_AttributeRef, AttributeValue);
    type IntoIter = AttributeIterator;

    fn into_iter(self) -> Self::IntoIter {
        AttributeIterator::new(self)
    }
}

impl Handle<OTF2_AttributeList> {
    pub fn length(&self) -> u32 {
        unsafe { OTF2_AttributeList_GetNumberOfElements(self.as_ptr()) }
    }

    fn get_attribute_by_index(&self, index: u32) -> Status<(OTF2_AttributeRef, AttributeValue)> {
        use OTF2_Type_enum::*;
        use AttributeValue::*;
        let mut attribute: OTF2_AttributeRef = OTF2_UNDEFINED_ATTRIBUTE;
        let mut kind: OTF2_Type = unsafe { std::mem::zeroed() };
        let mut value: OTF2_AttributeValue = unsafe { std::mem::zeroed() };
        unsafe { OTF2_AttributeList_GetAttributeByIndex(self.as_ptr(), index, &mut attribute, &mut kind, &mut value) }?;
        let value = match kind.to_enum() {
            OTF2_TYPE_NONE => None(()),
            OTF2_TYPE_UINT8 => Uint8(unsafe { value.0.uint8 }),
            OTF2_TYPE_UINT16 => Uint16(unsafe { value.0.uint16 }),
            OTF2_TYPE_UINT32 => Uint32(unsafe { value.0.uint32 }),
            OTF2_TYPE_UINT64 => Uint64(unsafe { value.0.uint64 }),
            OTF2_TYPE_INT8 => Int8(unsafe { value.0.int8 }),
            OTF2_TYPE_INT16 => Int16(unsafe { value.0.int16 }),
            OTF2_TYPE_INT32 => Int32(unsafe { value.0.int32 }),
            OTF2_TYPE_INT64 => Int64(unsafe { value.0.int64 }),
            OTF2_TYPE_FLOAT => Float32(unsafe { value.0.float32 }),
            OTF2_TYPE_DOUBLE => Float64(unsafe { value.0.float64 }),
            OTF2_TYPE_STRING => String(unsafe { value.0.stringRef }),
            OTF2_TYPE_ATTRIBUTE => Attribute(unsafe { value.0.attributeRef }),
            OTF2_TYPE_LOCATION => Location(unsafe { value.0.locationRef }),
            OTF2_TYPE_REGION => Region(unsafe { value.0.regionRef }),
            OTF2_TYPE_GROUP => Group(unsafe { value.0.groupRef }),
            OTF2_TYPE_METRIC => Metric(unsafe { value.0.metricRef }),
            OTF2_TYPE_COMM => Comm(unsafe { value.0.commRef }),
            OTF2_TYPE_PARAMETER => Parameter(unsafe { value.0.parameterRef }),
            OTF2_TYPE_RMA_WIN => RmaWin(unsafe { value.0.rmaWinRef }),
            OTF2_TYPE_SOURCE_CODE_LOCATION => SourceCodeLocation(unsafe { value.0.sourceCodeLocationRef }),
            OTF2_TYPE_CALLING_CONTEXT => CallingContext(unsafe { value.0.callingContextRef }),
            OTF2_TYPE_INTERRUPT_GENERATOR => InterruptGenerator(unsafe { value.0.interruptGeneratorRef }),
            OTF2_TYPE_IO_FILE => IoFile(unsafe { value.0.ioFileRef }),
            OTF2_TYPE_IO_HANDLE => IoHandle(unsafe { value.0.ioHandleRef }),
            OTF2_TYPE_LOCATION_GROUP => LocationGroup(unsafe { value.0.locationGroupRef }),
        };
        Ok((attribute, value))
    }
}

macro_rules! declare_and_impl_enum {
    ($enum:tt, $($variant:ident => $ty:ty),*) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $enum {
            $(
                $variant($ty),
            )*
        }

        impl $enum {
            pub fn type_name(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant(_) => stringify!($ty),
                    )*
                }
            }
        }

        $(
            impl TryFrom<$enum> for $ty {
                type Error = String;

                fn try_from(value: $enum) -> Result<Self, Self::Error> {
                    if let $enum::$variant(v) = value {
                        Ok(v)
                    } else {
                        Err(format!("{} expected {} but got {}", stringify!(TryFrom<$enum>), stringify!($ty), value.type_name()))
                    }
                }
            }
        )*
    };
}

declare_and_impl_enum!(AttributeValue,
    None => (),
    Uint8 => u8,
    Uint16 => u16,
    Uint32 => u32,
    Uint64 => u64,
    Int8 => i8,
    Int16 => i16,
    Int32 => i32,
    Int64 => i64,
    Float32 => f32,
    Float64 => f64,
    String => OTF2_StringRef,
    Attribute => OTF2_AttributeRef,
    Location => OTF2_LocationRef,
    Region => OTF2_RegionRef,
    Group => OTF2_GroupRef,
    Metric => OTF2_MetricRef,
    Comm => OTF2_CommRef,
    Parameter => OTF2_ParameterRef,
    RmaWin => OTF2_RmaWinRef,
    SourceCodeLocation => OTF2_SourceCodeLocationRef,
    CallingContext => OTF2_CallingContextRef,
    InterruptGenerator => OTF2_InterruptGeneratorRef,
    IoFile => OTF2_IoFileRef,
    IoHandle => OTF2_IoHandleRef,
    LocationGroup => OTF2_LocationGroupRef
);
