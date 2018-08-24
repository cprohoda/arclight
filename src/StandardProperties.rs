#[macro_use] extern crate lazy_static;
extern crate regex;

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

    fn read_dependency(&self, dependency: String) -> Result<(CrateName, Version), DependencyErr> {
        lazy_static! {
            static ref dependency_pattern: Regex = Regex::new(r"(?P<name>[a-zA-Z0-9]*):(?P<major>[0-9]*)\.(?P<minor>[0-9]*\.(?P<patch>[0-9]*)").unwrap();
        }

        let captures = dependency_pattern.captures(dependency).unwrap();
        let crate_name = captures.name("name");
        let major = captures.name("major");
        let minor = captures.name("minor");
        let patch = captures.name("patch");

        if crate_name.is_some() && major.is_some() && minor.is_some() && patch.is_some() {
            let major_version = usize::from_str_radix(major.unwrap(), 10);
            let minor_version = usize::from_str_radix(minor.unwrap(), 10);
            let patch_version = usize::from_str_radix(patch.unwrap(), 10);

            if major_version.is_ok() && minor_version.is_ok() && patch_version.is_ok() {
                Ok((CrateName::new(crate_name), Version::new(major_version, minor_version, patch_version))
            } else {
                Err(DependencyErr::VersionReadErr("Requires dependency format of \"crate_name:major.minor.patch\", where major, minor, and patch are symantic versions."))
            }
        } else {
            Err(DependencyErr::VersionReadErr("Requires dependency format of \"crate_name:major.minor.patch\", where major, minor, and patch are symantic versions."))
        }
    }
}

impl ResolvableProperty for Dependencies {
    pub fn resolve(&mut self, input: String) -> Result<(), PropertyErr> {
        if input.len() >= 1 {
            Ok(())
        } else {
            Err(DependencyErr::DependencyResolveErr("Failed to resolve"))
        }
    }
}

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

#[derive(Debug)]
enum DependencyErr {
    VersionReadErr,
    DependencyResolveErr,
}
