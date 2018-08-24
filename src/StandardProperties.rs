use Property::ResolvableProperty;
use Property::PropertyErr;
use ArclightSyntaxTree::ArclightSyntaxTree;

use std::collections::HashMap;
use regex::Regex;

pub struct Dependencies {
    used: HashMap<CrateName, Vec<Version>>,
    blacklisted: HashMap<CrateName, Vec<Version>>,
}

impl Dependencies {
    pub fn new() -> Dependencies {
        Dependencies {
            used: HashMap::new(),
            blacklisted: HashMap::new(),
        }
    }

    fn read_dependency(&self, dependency: String) -> Result<(CrateName, Version), PropertyErr> {
        lazy_static! {
            static ref dependency_pattern: Regex = Regex::new(r"(?P<name>[a-zA-Z0-9]*):(?P<major>[0-9]*)\.(?P<minor>[0-9]*\.(?P<patch>[0-9]*)").unwrap();
        }

        let captures = dependency_pattern.captures(&dependency).unwrap();
        let crate_name = captures.name("name");
        let major = captures.name("major");
        let minor = captures.name("minor");
        let patch = captures.name("patch");

        if crate_name.is_some() && major.is_some() && minor.is_some() && patch.is_some() {
            let major_version = usize::from_str_radix(major.map_or("", |m| m.as_str()), 10);
            let minor_version = usize::from_str_radix(minor.map_or("", |m| m.as_str()), 10);
            let patch_version = usize::from_str_radix(patch.map_or("", |m| m.as_str()), 10);

            if major_version.is_ok() && minor_version.is_ok() && patch_version.is_ok() {
                let resolved_dependency = (
                    CrateName::new(crate_name.map_or("".to_string(), |m| m.as_str().to_string())), 
                    Version::new(
                        major_version.expect("Unexpected error from unwrapping usize from major"),
                        minor_version.expect("Unexpected error from unwrapping usize from minor"),
                        patch_version.expect("Unexpected error from unwrapping usize from patch")
                    ));
                Ok(resolved_dependency)
            } else {
                Err(PropertyErr::DependencyParse("".to_string()))
            }
        } else {
            Err(PropertyErr::DependencyParse("".to_string()))
        }
    }
}

impl ResolvableProperty for Dependencies {
    fn resolve(&self) -> Result<(), PropertyErr> {
        Ok(())
    }
}

#[derive(Hash,PartialEq,Eq)]
struct CrateName {
    name: String,
}

impl CrateName {
    fn new(name: String) -> CrateName {
        CrateName {
            name: name,
        }
    }
}

#[derive(Hash,PartialEq,Eq)]
struct Version {
    number: Vec<usize>,
}

impl Version {
    fn new(major: usize, minor: usize, patch: usize) -> Version {
        Version {
            number: vec![major, minor, patch],
        }
    }
}
