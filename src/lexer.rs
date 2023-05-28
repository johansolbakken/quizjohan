use crate::quiz;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Text,
    Heading(u8),
    NewLine,
    Quiz(quiz::QuizStruct),
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub struct Lexer {
    input: String,
    current_token: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.to_string(),
            current_token: 0,
        }
    }

    pub fn lex(&mut self) -> Token {
        self.trim_whitespace_from_left();

        if self.current_token >= self.input.chars().count() {
            return Token {
                token_type: TokenType::EOF,
                value: String::new(),
            };
        }

        let current_char = self.input.chars().nth(self.current_token).unwrap();

        match current_char {
            '#' => {
                let heading_level = self.count_heading_level();
                Token {
                    token_type: TokenType::Heading(heading_level),
                    value: String::from("#".repeat(heading_level as usize)),
                }
            }
            '\n' => {
                self.current_token += 1;
                Token {
                    token_type: TokenType::NewLine,
                    value: String::from("\n"),
                }
            }
            // Add more cases for other token types
            _ => Token {
                token_type: TokenType::Text,
                value: self.text(),
            },
        }
    }

    // Helper method to count the number of '#' characters for heading level
    fn count_heading_level(&mut self) -> u8 {
        let mut count = 1;
        let mut index = self.current_token + 1;
        while index < self.input.len() {
            let char_at_index = self.input.chars().nth(index).unwrap();
            if char_at_index == '#' {
                count += 1;
                index += 1;
            } else {
                break;
            }
        }
        self.current_token = index;
        count
    }

    // getting text until newline
    fn text(&mut self) -> String {
        let mut text = String::new();
        let mut index = self.current_token;
        while index < self.input.len() {
            if index >= self.input.len() {
                break;
            }
            let char_at_index = self.input.chars().nth(index).unwrap();
            if char_at_index == '\n' {
                break;
            } else {
                text.push(char_at_index);
                index += 1;
            }
        }
        self.current_token = index;
        text
    }

    fn trim_whitespace_from_left(&mut self) {
        let mut index = self.current_token;
        while index < self.input.len() {
            if index >= self.input.chars().count() {
                break;
            }
            let char_at_index = self.input.chars().nth(index).unwrap();
            if char_at_index.is_whitespace() {
                index += 1;
            } else {
                break;
            }
        }
        self.current_token = index;
    }
}
