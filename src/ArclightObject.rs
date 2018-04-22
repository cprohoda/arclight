use Preset::Preset;

struct ArclightObject {
    properties: Vec<Property>
}

impl ArclightObject {
    fn new() -> ArclightObject {
        ArclightObject {
            properties: Vec::new(),
        }
    }

    fn push<T: ResolvableProperty>(&mut self, property: T) {
        self.properties.push(property);
    }

    fn resolve(&mut self) -> Result<(),PropertyErr> {
        for property in self.properties {
            match property.resolve() {
                Ok(()) => {},
                Err(e) => {return Err(e);},
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Property {
    type: String,
    value: String,
}

trait ResolvableProperty {
    fn resolve(&self) -> Result<(),PropertyErr>;
}