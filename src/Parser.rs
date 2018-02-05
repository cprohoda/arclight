const SPACE: char = ' ';
const TAB: char = '\t';
const NEW: char = '\n';
const INSTANCE_SEND: char = '<';
const INSTANCE_RETURN: char = '.';
const UNINST_RETURN: char = ':';
const ESCAPE: char = '\\';
const QUOTE: char = '\"';
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
    let mut parsed = Parsed::new();
    let mut input_chars = input.chars();

    while let Some(character) = input_chars.next() {
        match character {
            SPACE => {parsed.push_if(accumulator); accumulator = "".to_string();},
            NEW => {parsed.push_if(accumulator); parsed.push_if(NEW.to_string()); accumulator = "".to_string();},
            TAB => {parsed.push_if(accumulator); parsed.push_if(TAB.to_string()); accumulator = "".to_string();},
            ESCAPE => {input_chars.next();},
            QUOTE => {parsed.push_if(accumulator); accumulator = "\"".to_string();
                while let Some(quote_char) = input_chars.next() {
                    match quote_char {
                        ESCAPE => {input_chars.next();},
                        QUOTE => {accumulator.push(quote_char); parsed.push_if(accumulator); accumulator = "".to_string(); break;}
                        _ => {accumulator.push(quote_char);},
                    }
                }
            },
            _ => accumulator.push(character),
        }
    }

    parsed.push_if(accumulator);
    return parsed.data();
}

#[derive(Clone)]
struct Parsed {
    data: Vec<String>,
}

impl Parsed {
    fn new() -> Parsed {
        Parsed {
            data: Vec::new()
        }
    }

    fn push_if(&mut self, value: String) {
        if value != "" {
            self.data.push(value);
        }
    }

    fn data(&self) -> Vec<String> {
        self.data.clone()
    }
}