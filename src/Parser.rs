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
        Err(e) => {
            return Err(e);
        },
        Ok(()) => {
            tokens.finalize();
            return Ok(tokens);
        },
    }
}

#[derive(Debug,PartialEq)]
pub enum ParserError {
    UnexpectedSpace,
    UnmatchedQuote,
    UnmatchedParen,
    UnterminatedBranch,
}

#[derive(PartialEq)]
pub enum TokenType {
    Control,
    Pass,
    Return,
    Defined,
    Photon,
}

pub struct Token {
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

        const PASS_STR: &str = "<";
        const RETURN_STR: &str = ".";
        const DEFINED_STR: &str = ":";

        match token.as_str() {
            PASS_STR => TokenType::Pass,
            RETURN_STR => TokenType::Return,
            DEFINED_STR => TokenType::Defined,
            _ => {
                for character in token.chars() {
                    if !(character == NEW || character == TAB) {
                        return TokenType::Photon;
                    }
                };
                return TokenType::Control;
            },
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token_type_name = match self.token_type {
            TokenType::Control => "Control",
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

        Ok(())
    }

    pub fn finalize(&mut self) {
        self.push_token_from_accumulator();
    }

    fn character_match(&mut self, character: char, input_chars: &mut Chars) -> Result<(),ParserError> {
        match character {
            SPACE => {
                if self.accumulator == "".to_string() {
                    return Err(ParserError::UnexpectedSpace);
                } else {
                    self.push_token_from_accumulator();
                    return Ok(());
                }
            },
            NEW => {
                self.control_handler(NEW)
            },
            TAB => {
                self.control_handler(TAB)
            },
            ESCAPE => {
                self.accumulator.push(character);
                self.accumulator.push(input_chars.next().unwrap());
                Ok(())
            },
            PASS => {
                self.push_token_from_accumulator();
                self.push_character_token(PASS);
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

    fn control_handler(&mut self, control: char) -> Result<(),ParserError> {
        let mut is_accumulator_control = true;
        for character in self.accumulator.chars() {
            if !(character == NEW || character == TAB) {
                is_accumulator_control = false;
            }
        }
        if !is_accumulator_control {
            self.push_token_from_accumulator();
        }
        self.accumulator.push(control);
        Ok(())
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
        let accumulated_token = Token::new(self.accumulator.to_owned());
        self.Tokens.push(accumulated_token);
        self.accumulator = "".to_string();
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
    use Parser::PASS;

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
    fn quote_parse() {
        let mut expected = Tokens::new();
        expected.push_token(Token {
            token: "a".to_string(),
            token_type: TokenType::Photon,
        });
        expected.push_token(Token {
            token: "\"giant\tapple\"".to_string(),
            token_type: TokenType::Photon,
        });

        let actual = parse("a \"giant\tapple\"").expect("Testing quote_parse, parse");
        assert_eq!(expected, actual);
    }

    #[test]
    fn pass_parse() {
        let mut expected = Tokens::new();
        expected.push_token(Token {
            token: "a".to_string(),
            token_type: TokenType::Photon,
        });
        expected.push_token(Token {
            token: PASS.to_string(),
            token_type: TokenType::Pass,
        });
        expected.push_token(Token {
            token: "b".to_string(),
            token_type: TokenType::Photon,
        });

        let actual = parse("a<b").expect("Testing pass_parse, parse");
        assert_eq!(expected, actual);
    }

    #[test]
    fn control_parse() {
        let mut expected = Tokens::new();
        expected.push_token(Token {
            token: "a".to_string(),
            token_type: TokenType::Photon,
        });
        expected.push_token(Token {
            token: "\n\n".to_string(),
            token_type: TokenType::Control,
        });
        expected.push_token(Token {
            token: "b".to_string(),
            token_type: TokenType::Photon,
        });
        expected.push_token(Token {
            token: "\t\n\n".to_string(),
            token_type: TokenType::Control,
        });
        expected.push_token(Token {
            token: "a".to_string(),
            token_type: TokenType::Photon,
        });

        let actual = parse("a\n\nb\t\n\na").expect("Testing control_parse, parse");
        assert_eq!(expected, actual);
    }

    #[test]
    fn double_space_parse() {
        assert_eq!(ParserError::UnexpectedSpace, parse("a  b").unwrap_err());
    }

    #[test]
    fn beginning_space_parse() {
        assert_eq!(ParserError::UnexpectedSpace, parse(" a").unwrap_err());
    }

    #[test]
    fn ending_space_parse() { // TODO: implement
        assert_eq!(ParserError::UnterminatedBranch, parse("a ").unwrap_err());
    }

    #[test]
    fn unmatched_paren_parse() { // TODO: implement
        assert_eq!(ParserError::UnmatchedParen, parse("(a (b )").unwrap_err());
    }

    #[test]
    fn unmatched_quote_parse() { // TODO: implement
        assert_eq!(ParserError::UnmatchedQuote, parse("a \"b").unwrap_err());
    }

    #[test]
    fn patial_tokenize(){
        let mut partial = Tokens::new();
        partial.tokenize("ab").expect("Testing partial_tokenize, first tokenize");
        partial.tokenize("c ").expect("Testing partial_tokenize, second tokenize");
        partial.tokenize("d").expect("Testing partial_tokenize, third tokenize");
        partial.finalize();

        let full = parse("abc d").expect("Testing partial_tokenize, parse");
        assert_eq!(partial, full);
    }
}
