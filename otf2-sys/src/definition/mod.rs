mod attribute;
mod location;
mod global_reader_callbacks;
mod string;
mod visitor;

pub use global_reader_callbacks::GlobalDefReaderCallbacks;
pub use visitor::DefinitionVisitor;
pub use string::StringRegistry;
pub use attribute::AttributeRegistry;
pub use location::LocationRegistry;

#[derive(Debug, Default)]
pub struct Definitions {
    pub locations: LocationRegistry,
    pub strings: StringRegistry,
    pub attributes: AttributeRegistry,
}
