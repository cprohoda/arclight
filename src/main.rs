#[macro_use] extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;
use std::io::{Read, Write};

mod Parser;
mod ArclightSyntaxTree;
mod Photon;
mod Preset;
mod Property;
mod ActiveProperties;
mod StandardProperties;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap().as_str();
    let path = fs::canonicalize(filename).expect("");
    let path_str = path.to_str().unwrap();

    let mut f_in = fs::File::open(path_str).expect("Failed to find input file");
    let mut contents = String::new();
    f_in.read_to_string(&mut contents).expect("Failed to read input file contents");

    let tokens = Parser::parse(&contents).unwrap();
    let mut ast = ArclightSyntaxTree::ArclightSyntaxTree::new();
    ast.build_at_marker(tokens);

    let mut compiled = ast.generate().expect("Failed to generate arclight");

    let mut f_out = fs::File::create(path_str.to_owned() + ".al").unwrap();
    f_out.write(compiled.as_bytes());
    f_out.flush();
}