use Property::ResolvableProperty;
use Property::PropertyErr;
use ArclightSyntaxTree::ArclightSyntaxTree;

pub struct ArclightObject {
    properties: Vec<Box<ResolvableProperty>>,
    token: String,
}

impl ArclightObject {
    fn new() -> ArclightObject {
        ArclightObject {
            properties: Vec::new(),
            token: "".to_string(),
        }
    }

    fn push(&mut self, property: impl ResolvableProperty + 'static) {
        self.properties.push(Box::new(property));
    }

    fn resolve(&mut self) -> Result<(),PropertyErr> {
        for property in self.properties {
            property.resolve()?;
        }
        Ok(())
    }
}
