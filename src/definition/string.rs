use crate::internal::*;
use std::ffi::{CStr, CString};

use super::DefinitionVisitor;

pub type StringRegistry = std::collections::BTreeMap<OTF2_StringRef, String>;

impl DefinitionVisitor for StringRegistry {
    fn visit_string(&mut self, defn: OTF2_StringRef, value: &CStr) -> OTF2_CallbackCode {
        self.insert(defn, value.to_string_lossy().into_owned());
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }
}
