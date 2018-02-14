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
                },
                PAREN_OPEN => {
                    self.paren_parse(input_chars);
                },
                _ => self.accumulator.push(character),
            }
        };
    }

    fn paren_parse(&mut self, input_chars: Chars) {
        self.push_token_from_accumulator(); // TODO should it be parsing error? Similar to quote case above
        self.accumulator.push(PAREN_OPEN);
        while let Some(char_in_paren) = input_chars.next() {
            // 
            // maybe use a macro to apply same matching rules here as in tokenize for SPACE, NEW, TAB, ESCAPE, QUOTE, and PAREN_OPEN with a separate addition rule for PAREN_CLOSE?
        }
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

