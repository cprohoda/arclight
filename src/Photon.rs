use std::fmt;

use Property::ResolvableProperty;
use Property::PropertyErr;

pub struct Photon {
    pub down: Option<usize>,
    pub up: Option<usize>,
    pub right: Option<usize>,
    pub left: Option<usize>,
    pub token: String,
    pub properties: Vec<Box<ResolvableProperty>>,
}

impl Photon {
    pub fn new(token: String) -> Photon {
        Photon{
            down: None,
            up: None,
            right: None,
            left: None,
            token: "".to_string(),
            properties: Vec::new(),
        }
    }

    pub fn push_property(&mut self, property: impl ResolvableProperty + 'static) {
        self.properties.push(Box::new(property));
    }

    pub fn push_to_token(&mut self, partial_token: String) {
        self.token.push_str(partial_token.as_str());
    }
}

impl fmt::Debug for Photon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token: {:?}\nProperties: {:?}", self.token, self.properties)
    }
}

impl PartialEq for Photon {
    fn eq(&self, other: &Photon) -> bool {
        let mut equality: bool = self.properties.len() == other.properties.len();
        let mut iter_other = other.properties.iter();

        for self_token in &self.properties {
            if !equality { break; }
            equality = self_token == iter_other.next().unwrap();
        }

        equality
    }
}
