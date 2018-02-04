const SPACE: char = ' ';
const TAB: char = '\t';
const NEW: char = '\n';
const INSTANCE_SEND: char = '<';
const INSTANCE_RETURN: char = '.';
const UNINST_RETURN: char = ':';
const ESCAPE: char = '\\';
const QUOTE: char = '"';
const PAREN_OPEN: char = '(';
const PAREN_CLOSE: char = ')';

enum state {
    escape,
    quote,
    paren,
    standard,
}

pub fn parse(input: &str) -> Vec<String> {
    let mut accumulator = "".to_string();
    let mut parsed: Vec<String> = Vec::new();

    for character in input.chars() {
        match character {
            SPACE => {parsed.push(accumulator); accumulator = "".to_string();},
            NEW => {parsed.push(accumulator); parsed.push(NEW.to_string()); accumulator = "".to_string();},
            TAB => {parsed.push(accumulator); parsed.push(TAB.to_string()); accumulator = "".to_string();},  
            _ => accumulator.push(character),
        }
    }
    parsed.push(accumulator);
    return parsed;
}