use Property::{ResolvableProperty, PropertyErr};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub struct ActiveProperties {
    properties: HashMap<Property, usize>,
}

impl ActiveProperties {
    pub fn new() -> ActiveProperties {
        ActiveProperties {
            properties: HashMap::new(),
        }
    }
}

struct Property {
    property: Box<ResolvableProperty>,
    crate_name: String,
    property_name: String,
}

impl Property {
    fn new(property: Box<ResolvableProperty>, crate_name: impl Into<String>, property_name: impl Into<String>) -> Property {
        Property {
            property: property,
            crate_name: crate_name.into(),
            property_name: property_name.into(),
        }
    }

    fn resolve(&self) -> Result<(),PropertyErr> {
        self.property.resolve()
    }
}

impl Hash for Property {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.crate_name.hash(state);
        self.property_name.hash(state);
    }
}

impl PartialEq for Property {
    fn eq(&self, other: &Property) -> bool {
        self.crate_name == other.crate_name && self.property_name == other.property_name
    }
}

impl Eq for Property {}
