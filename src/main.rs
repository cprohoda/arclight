#![feature(generators)]
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod Parser;

fn main() {
	let filename = "~/arclight/arclight_string_examples";
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("Failed to read the file");

    let blah = Parser::parse(&contents);
}