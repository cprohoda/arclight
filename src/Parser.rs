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

pub fn parse(input: &str) -> Result<Tokens, ParserError> {
    let mut tokens = Tokens::new();
    match tokens.tokenize(input) {
        Err(e) => {return Err(e);},
        Ok(()) => {return Ok(tokens);},
    }
}

#[derive(Debug,PartialEq)]
pub enum ParserError {
    DoubleSpace,
    UnmatchedQuote,
    UnmatchedParen,
    EndingSpace,
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

        const NEW_STR: &str = "\n";
        const TAB_STR: &str = "\t";
        const PASS_STR: &str = "<";
        const RETURN_STR: &str = ".";
        const DEFINED_STR: &str = ":";

        match token.as_str() {
            NEW_STR => TokenType::New,
            TAB_STR => TokenType::Tab,
            PASS_STR => TokenType::Pass,
            RETURN_STR => TokenType::Return,
            DEFINED_STR => TokenType::Defined,
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

    pub fn tokenize(&mut self, input: &str) -> Result<(), ParserError> {
        let mut input_chars = input.chars();

        while let Some(character) = input_chars.next() {
            match self.character_match(character, &mut input_chars) {
                Err(e) => {return Err(e);},
                Ok(()) => {},
            }
        };
        self.push_token_from_accumulator();

        Ok(())
    }

    fn character_match(&mut self, character: char, input_chars: &mut Chars) -> Result<(),ParserError> {
        match character {
            SPACE => {
                self.push_token_from_accumulator();
                Ok(())
            },
            NEW => {
                self.push_token_from_accumulator();
                self.push_character_token(NEW);
                Ok(())
            },
            TAB => {
                self.push_token_from_accumulator();
                self.push_character_token(TAB);
                Ok(())
            },
            ESCAPE => {
                self.accumulator.push(character);
                self.accumulator.push(input_chars.next().unwrap());
                Ok(())
            },
            QUOTE => {
                self.quote_match(input_chars);
                Ok(())
            },
            PAREN_OPEN => {
                self.paren_open_match(input_chars);
                Ok(())
            },
            _ => {
                self.accumulator.push(character);
                Ok(())
            },
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
    use Parser::ParserError;

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

        let actual = parse("a b").expect("This shouldn't error");
        assert_eq!(expected, actual);
    }

    #[test]
    fn double_space_parse() { // TODO: implement this behavior
        assert_eq!(ParserError::DoubleSpace, parse("a  b").unwrap_err());
    }

    fn ending_space_parse() { // TODO: implement
        assert_eq!(ParserError::EndingSpace, parse("a ").unwrap_err());
    }

    fn unmatched_paren_parse() { // TODO: implement
        assert_eq!(ParserError::UnmatchedParen, parse("(a (b )").unwrap_err());
    }

    fn unmatched_quote_parse() { // TODO: implement
        assert_eq!(ParserError::UnmatchedQuote, parse("a \"b").unwrap_err());
    }
}
