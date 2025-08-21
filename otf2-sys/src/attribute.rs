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
        let mut attribute: OTF2_AttributeRef = OTF2_UNDEFINED_ATTRIBUTE;
        let mut kind: OTF2_Type = unsafe { std::mem::zeroed() };
        let mut value: OTF2_AttributeValue = unsafe { std::mem::zeroed() };
        unsafe {
            OTF2_AttributeList_GetAttributeByIndex(
                self.as_ptr(),
                index,
                &mut attribute,
                &mut kind,
                &mut value,
            )
        }?;
        Ok((attribute, AttributeValue::new(kind, value)))
    }
}

declare_enum_union_wrapper!(
    #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
    pub enum AttributeValue(union: OTF2_AttributeValue) {
        OTF2_TYPE_NONE => None(()),
        OTF2_TYPE_UINT8 => Uint8(u8) from uint8,
        OTF2_TYPE_UINT16 => Uint16(u16) from uint16,
        OTF2_TYPE_UINT32 => Uint32(u32) from uint32,
        OTF2_TYPE_UINT64 => Uint64(u64) from uint64,
        OTF2_TYPE_INT8 => Int8(i8) from int8,
        OTF2_TYPE_INT16 => Int16(i16) from int16,
        OTF2_TYPE_INT32 => Int32(i32) from int32,
        OTF2_TYPE_INT64 => Int64(i64) from int64,
        OTF2_TYPE_FLOAT => Float32(f32) from float32,
        OTF2_TYPE_DOUBLE => Float64(f64) from float64,
        OTF2_TYPE_STRING => String(OTF2_StringRef) from stringRef,
        OTF2_TYPE_ATTRIBUTE => Attribute(OTF2_AttributeRef) from attributeRef,
        OTF2_TYPE_LOCATION => Location(OTF2_LocationRef) from locationRef,
        OTF2_TYPE_REGION => Region(OTF2_RegionRef) from regionRef,
        OTF2_TYPE_GROUP => Group(OTF2_GroupRef) from groupRef,
        OTF2_TYPE_METRIC => Metric(OTF2_MetricRef) from metricRef,
        OTF2_TYPE_COMM => Comm(OTF2_CommRef) from commRef,
        OTF2_TYPE_PARAMETER => Parameter(OTF2_ParameterRef) from parameterRef,
        OTF2_TYPE_RMA_WIN => RmaWin(OTF2_RmaWinRef) from rmaWinRef,
        OTF2_TYPE_SOURCE_CODE_LOCATION => SourceCodeLocation(OTF2_SourceCodeLocationRef) from sourceCodeLocationRef,
        OTF2_TYPE_CALLING_CONTEXT => CallingContext(OTF2_CallingContextRef) from callingContextRef,
        OTF2_TYPE_INTERRUPT_GENERATOR => InterruptGenerator(OTF2_InterruptGeneratorRef) from interruptGeneratorRef,
        OTF2_TYPE_IO_FILE => IoFile(OTF2_IoFileRef) from ioFileRef,
        OTF2_TYPE_IO_HANDLE => IoHandle(OTF2_IoHandleRef) from ioHandleRef,
        OTF2_TYPE_LOCATION_GROUP => LocationGroup(OTF2_LocationGroupRef) from locationGroupRef,
    }
);
