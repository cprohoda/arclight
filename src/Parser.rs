#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};

// let specials = 

const space: char = ' ';
const tab: char = '\t';
const new: char = '\n';
const instance_send: char = '<';
const instance_return: char = '.';
const uninst_return: char = ':';
const escape: char = '\\';
const quote: char = '"';
const paren_open: char = '(';
const paren_close: char = ')';

// enum ParseResult {}

pub fn parse(input: &str) -> Result<Option<String>, E> {
    let mut accumulator = "".to_string();

    let mut generator = || {
        accumulator = "".to_string(); 
        for character in input.chars() {
            accumulator.push(character);
            match character {
                space => yield accumulator,
                new => yield accumulator,
                _ => {},
            }
        }
        return None
    };
}