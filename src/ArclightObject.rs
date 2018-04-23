use Preset::DefaulPresetType;

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
    value: String,
    preset: Box<Property>,
}

impl ResolvableProperty for Property {
    type PresetType = DefaulPresetType;
    // default properties
    pub fn resolve(&self) -> Result<(), PropertyErr> {
        Ok(())
    }

    pub fn current(&self) -> DefaulPresetType {
        DefaulPresetType::default
    }

    pub fn set(&mut self, preset: Property) -> Result<(), PropertyErr> {
        self.preset = Box<preset>;
        Ok(())
    }
}

trait ResolvableProperty {
    type PresetType;

    pub fn resolve(&self) -> Result<(),PropertyErr>;
    pub fn current(&self) -> Self::PresetType;
    pub fn set(&mut self, Self::PresetType) -> Result<(),PropertyErr>;
}

pub enum PropertyErr {
    SetFailed,
    ResolveFailed,
}