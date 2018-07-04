use Property::ResolvableProperty;
use Property::PropertyErr;
use ArclightSyntaxTree::ArclightSyntaxTree;

use std::collections::HashMap;
use regex::Regex;

pub struct Dependency {
    dependency_versions: HashMap<CrateName, Vec<Version>>, 
}

pub struct CrateName {
    name: Vec<String>,
}

impl ResolvableProperty for CrateName {
    let crate_name_pattern = Regex::new(r"[a-zA-Z0-9:]").unwrap();

    pub fn resolve(&mut self, input: String) -> Result<(), PropertyErr> {
        if input.len() >= 1 && crate_name_pattern.is_match(input.as_str()) {
            let split_name = input.split(':');
            self.name = split_name;
            Ok(())
        } else {
            Err(PropertyErr::CrateNameParse("Failed to parse crate name: ".to_String() + input))
        }
    }
}

pub struct Version {
    number: Vec<String>,
    marker: usize,
}

impl ResolvableProperty for Version {
    let version_pattern = Regex::new(r"[0-9\.]").unwrap();

    pub fn resolve(&mut self, input: String, marker: usize) -> Result<(), PropertyErr> {
        if input.len() >= 1 && version_pattern.is_match(input.as_str()) {
            let split_version = input.split('.');
            self.number = split_version;
            self.marker = marker;
            Ok(())
        } else {
            Err(PropertyErr::CrateNameParse("Failed to parse crate name: ".to_String() + input))
        }
    }
}