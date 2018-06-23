use std::fmt;

pub trait ResolvableProperty {
    fn resolve(&self) -> Result<(),PropertyErr>;
}

// TODO: figure out how we want to Debug and PartialEq for ResolvableProperty
impl fmt::Debug for ResolvableProperty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl PartialEq for ResolvableProperty {
    fn eq(&self, other: &ResolvableProperty) -> bool {
        true
    }
}

pub enum PropertyErr {
    ResolveFailed(String),
}
