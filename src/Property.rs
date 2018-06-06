pub trait ResolvableProperty {
    fn resolve(&self) -> Result<(),PropertyErr>;
}

pub enum PropertyErr {
    ResolveFailed(String),
}
