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
    type: TokenType,
}

impl Token {
    fn new(token: &String) -> Token {
        Token {
            token: token,
            token_type: derive_type(token),
        }
    }

    fn derive_type(token: String) -> TokenType {
        match token {
            NEW => TokenType::New,
            TAB => TokenType::Tab,
            PASS => TokenType::Pass,
            RETURN => TokenType::Return,
            DEFINED => TokenType::Defined,
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
            accumulator: "",
        }
    }

    pub fn tokenize(&mut self, input: &str) {
        let mut input_chars = input.chars();

        while let Some(character) = input_chars.next() {
            self.character_match(character, input_chars, self.other_char_match);
        };
    }

    fn character_match(&mut self, character: char, input_chars: Chars, final_branch: &Fn(char)) {
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
                self.accumulator.push(input_chars.next());
            },
            QUOTE => {
                self.quote_match(input_chars);
            },
            PAREN_OPEN => {
                self.paren_open_match(input_chars);
            },
            _ => {
                final_branch(character);
            },
        }
    }

    fn quote_match(&mut self, mut input_chars: Chars) {
        self.push_token_from_accumulator(); // TODO should this be a parsing error if accumulator != ""? Can we start a quote in the middle of a token?
        self.accumulator.push(QUOTE.to_string());
        while let Some(char_in_quote) = input_chars.next() {
            self.accumulator.push(char_in_quote);
            match char_in_quote {
                ESCAPE => {
                    self.accumulator.push(input_chars.next());
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
            self.character_match(char_in_paren, input_chars, self.paren_close_match(character));
        }
    }

    fn paren_close_branch(character: char) {
        self.accumulator.push(character);
        match character {
            PAREN_CLOSE => {
                self.push_token_from_accumulator();
                break
            },
            _ => {},
        }
    }

    fn other_char_match(character: char) {
        self.accumulator.push(character)
    }

    fn push_token_from_accumulator(&mut self) -> Token {
        if self.accumulator != "" {
            let accumulated_token = Token::new(&self.accumulator);
            self.Tokens.push(accumulated_token);
            self.accumulator = "";
        }
    }

    fn push_character_token(&mut self, character: char) {
        let character_token = Token::new(&character.to_string());
        self.Tokens.push(character_token);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn space_parsing() {
        let mut expected = Tokens::new();
        expected.push(Token {
            token: "a",
            token_type: TokenType::Photon,
        });
        expected.push(Token {
            token: "b",
            token_type: TokenType::Photon,
        });

        let actual = parse("a b");

        assert_eq!(expected, actual); // TODO: Does this work? I assume I need to add a trait to Tokens
    }

    // #[test]
    // fn double_space_parse() { // TODO: this should throw parsing error
    //     let actual = parse("a  b");
    // }
}