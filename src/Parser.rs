use std::str::Chars;

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
            token: token,
            token_type: Token::derive_type(token),
        }
    }

    fn derive_type(token: String) -> TokenType {
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

struct Tokens {
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
            self.character_match(character, input_chars);
        };
    }

    fn character_match(&mut self, character: char, mut input_chars: Chars) {
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

    fn quote_match(&mut self, mut input_chars: Chars) {
        self.push_token_from_accumulator(); // TODO should this be a parsing error if accumulator != ""? Can we start a quote in the middle of a token?
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

    fn paren_open_match(&mut self, mut input_chars: Chars) {
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
        if self.accumulator != "" {
            let accumulated_token = Token::new(self.accumulator);
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

        // assert_eq!(expected, actual); // TODO: Does this work? I assume I need to add a trait to Tokens
    }

    // #[test]
    // fn double_space_parse() { // TODO: this should throw parsing error
    //     let actual = parse("a  b");
    // }
}