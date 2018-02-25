use std::str::Chars;
use std::fmt;

const SPACE: char = ' ';
const TAB: char = '\t';
const NEW: char = '\n';
const PASS: char = '<';
const RETURN: char = '.';
const DEFINED: char = ':';
const ESCAPE: char = '\\';
const QUOTE: char = '\"';
const PAREN_OPEN: char = '(';
const PAREN_CLOSE: char = ')';

pub fn parse(input: &str) -> Tokens {
    let mut tokens = Tokens::new();
    tokens.tokenize(input);

    tokens
}

#[derive(PartialEq)]
enum TokenType {
    New,
    Tab,
    Pass,
    Return,
    Defined,
    Photon,
}

struct Token {
    token: String,
    token_type: TokenType,
}

impl Token {
    fn new(token: String) -> Token {
        Token {
            token_type: Token::derive_type(&token),
            token: token,
        }
    }

    fn derive_type(token: &String) -> TokenType {
        use Parser::TokenType;

        let new_string = NEW.to_string();
        let tab_string = TAB.to_string();
        let pass_string = PASS.to_string();
        let return_string = RETURN.to_string();
        let defined_string = DEFINED.to_string();

        match token {
            new_string => TokenType::New,
            tab_string => TokenType::Tab,
            pass_string => TokenType::Pass,
            return_string => TokenType::Return,
            defined_string => TokenType::Defined,
            _ => TokenType::Photon,
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token_type_name = match self.token_type {
            TokenType::New => "New",
            TokenType::Tab => "Tab",
            TokenType::Pass => "Pass",
            TokenType::Return => "Return",
            TokenType::Defined => "Defined",
            TokenType::Photon => "Photon",
        };
        write!(f, "Token {:?}:{:?}", token_type_name, self.token)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.token_type == other.token_type && self.token == other.token
    }
}

pub struct Tokens {
    Tokens: Vec<Token>,
    accumulator: String,
}

impl Tokens {
    pub fn new() -> Tokens {
        Tokens {
            Tokens: Vec::new(),
            accumulator: "".to_string(),
        }
    }

    pub fn tokenize(&mut self, input: &str) {
        let mut input_chars = input.chars();

        while let Some(character) = input_chars.next() {
            self.character_match(character, &mut input_chars);
        };
    }

    fn character_match(&mut self, character: char, input_chars: &mut Chars) {
        match character {
            SPACE => {
                self.push_token_from_accumulator();
            },
            NEW => {
                self.push_token_from_accumulator();
                self.push_character_token(NEW);
            },
            TAB => {
                self.push_token_from_accumulator();
                self.push_character_token(TAB);
            },
            ESCAPE => {
                self.accumulator.push(character);
                self.accumulator.push(input_chars.next().unwrap());
            },
            QUOTE => {
                self.quote_match(input_chars);
            },
            PAREN_OPEN => {
                self.paren_open_match(input_chars);
            },
            _ => {},
        }
    }

    fn quote_match(&mut self, input_chars: &mut Chars) {
        self.push_token_from_accumulator(); // TODO should this be a parsing error if accumulator != "".to_string()? Can we start a quote in the middle of a token?
        self.accumulator.push(QUOTE);
        while let Some(char_in_quote) = input_chars.next() {
            self.accumulator.push(char_in_quote);
            match char_in_quote {
                ESCAPE => {
                    self.accumulator.push(input_chars.next().unwrap());
                },
                QUOTE => {
                    self.push_token_from_accumulator();
                    break;
                },
                _ => {},
            }
        }
    }

    fn paren_open_match(&mut self, input_chars: &mut Chars) {
        self.push_token_from_accumulator(); // TODO should it be parsing error? Similar to quote case above
        self.accumulator.push(PAREN_OPEN);
        while let Some(char_in_paren) = input_chars.next() {
            match char_in_paren {
                PAREN_CLOSE => {
                    self.accumulator.push(char_in_paren);
                    self.push_token_from_accumulator();
                    break;
                },
                _ => {
                    self.character_match(char_in_paren, input_chars);
                },
            };
        };
    }

    fn push_token_from_accumulator(&mut self) {
        if self.accumulator != "".to_string() {
            let accumulated_token = Token::new(self.accumulator.to_owned());
            self.Tokens.push(accumulated_token);
            self.accumulator = "".to_string();
        };
    }

    fn push_character_token(&mut self, character: char) {
        let character_token = Token::new(character.to_string());
        self.Tokens.push(character_token);
    }

    fn push_token(&mut self, token: Token) {
        self.Tokens.push(token);
    }
}

impl fmt::Debug for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tokens {:?}", self.Tokens)
    }
}

impl PartialEq for Tokens {
    fn eq(&self, other: &Tokens) -> bool {
        let mut equality: bool = self.Tokens.len() == other.Tokens.len();
        let mut iter_other = other.Tokens.iter();

        for self_token in &self.Tokens {
            if !equality { break; }
            equality = self_token == iter_other.next().unwrap();
        }

        equality
    }
}

#[cfg(test)]
mod tests {
    use Parser::parse;
    use Parser::Token;
    use Parser::Tokens;
    use Parser::TokenType;

    #[test]
    fn space_parsing() {
        let mut expected = Tokens::new();
        expected.push_token(Token {
            token: "a".to_string(),
            token_type: TokenType::Photon,
        });
        expected.push_token(Token {
            token: "b".to_string(),
            token_type: TokenType::Photon,
        });

        let actual = parse("a b");
        assert_eq!(expected, actual);
    }

    // #[test]
    // fn double_space_parse() { // TODO: this should throw parsing error
    //     let actual = parse("a  b");
    // }
}