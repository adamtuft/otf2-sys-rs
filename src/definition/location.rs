use crate::internal::*;
use std::ffi::{CStr, CString};

use super::DefinitionVisitor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocationDef {
    name: OTF2_StringRef,
    location_type: OTF2_LocationType,
    num_events: u64,
    location_group: OTF2_LocationGroupRef,
}

pub type LocationRegistry = std::collections::BTreeMap<OTF2_LocationRef, LocationDef>;

impl DefinitionVisitor for LocationRegistry {
    fn visit_location(&mut self, defn: OTF2_LocationRef, name: OTF2_StringRef, location_type: OTF2_LocationType, num_events: u64, location_group: OTF2_LocationGroupRef) -> OTF2_CallbackCode {
        self.insert(defn, LocationDef { name, location_type, num_events, location_group });
        OTF2_CallbackCode::OTF2_CALLBACK_SUCCESS
    }
}
