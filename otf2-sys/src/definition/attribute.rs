use crate::internal::*;
use std::ffi::{CStr, CString};

use super::DefinitionVisitor;
use super::defs::AttributeDef;

pub type AttributeRegistry = std::collections::BTreeMap<OTF2_AttributeRef, AttributeDef>;

impl DefinitionVisitor for AttributeRegistry {
    fn visit_attribute(&mut self, defn: OTF2_AttributeRef, name: OTF2_StringRef, description: OTF2_StringRef, type_: OTF2_Type) -> OTF2_CallbackCode {
        self.insert(defn, AttributeDef { name, description, kind: type_ });
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }
}
