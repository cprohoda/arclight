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

pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Tokens::new();
    let mut input_chars = input.chars();

    while let Some(character) = input_chars.next() {
        match character {
            SPACE => {tokens.push_if(accumulator); accumulator = "".to_string();},
            NEW => {tokens.push_if(accumulator); tokens.push_if(NEW.to_string()); accumulator = "".to_string();},
            TAB => {tokens.push_if(accumulator); tokens.push_if(TAB.to_string()); accumulator = "".to_string();},
            ESCAPE => {input_chars.next();},
            QUOTE => {tokens.push_if(accumulator); accumulator = "\"".to_string();
                while let Some(quote_char) = input_chars.next() {
                    match quote_char {
                        ESCAPE => {input_chars.next();},
                        QUOTE => {accumulator.push(quote_char); tokens.push_if(accumulator); accumulator = "".to_string(); break;}
                        _ => {accumulator.push(quote_char);},
                    }
                }
            },
            _ => accumulator.push(character),
        }
    }

    tokens.push_if(accumulator);
    return tokens.data();
}

enum TokenType {
    New,
    Tab,
    Quote,
    Photon,
    Arclight,
}

struct Token {
    token: String,
    token_type: TokenType, 
}

impl Token {
    fn new(token: String) -> Token {
        Token {
            token: token,
            token_type: derive_type(token),
        }
    }

    fn derive_type(token: String) -> TokenType {
        match token {
            TAB => 
            NEW =>
            INSTANCE_SEND =>
            INSTANCE_RETURN =>
            UNINST_RETURN =>
            ESCAPE =>
            QUOTE =>
            PAREN_OPEN =>
            PAREN_CLOSE =>
            _ => 
        }
    }
}

#[derive(Clone)]
struct Tokens {
    Tokens: Vec<Token>,
    accumulator: String,
}

impl Tokens {
    fn new() -> Tokens {
        Tokens {
            Tokens: Vec::new()
        }
    }

    fn push_if(&mut self, token: Token) {
        if token.token != "" {
            self.data.push(value);
        }
    }

    fn data(&self) -> Vec<String> {
        self.data.clone()
    }
}

