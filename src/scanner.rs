use std::collections::HashMap;
use crate::scanner::error::Error;

mod error;

enum TokenType {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,
    Not,

    LeftParen,
    RightParen,

    RightBracket,
    LeftBracket,

    LeftBrace,
    RightBrace,

    EqualsEquals,
    NotEquals,

    Let,
    Const,
    Fun,
}

struct Scanner {
    source: String,
    current_pos: usize,
    tokens: Vec<TokenType>,
    keywords: HashMap<String, TokenType>
}

impl Scanner {
    fn new(&mut self,src: String) -> Scanner {
        let scanner: Scanner = Scanner { source: src, current_pos: 0, tokens: vec![], keywords: HashMap::new() };

        self.keywords.insert(String::from("let"), TokenType::Let);
        self.keywords.insert(String::from("const"), TokenType::Const);
        self.keywords.insert(String::from("fun"), TokenType::Fun);

        return scanner;
    }

    fn scan_tokens(&mut self) {
        while self.current_pos < self.source.len() {
            self.next_token();
        }
    }

    fn add_token(&mut self, token: TokenType) {
        self.tokens.push(token);
    }

    fn next_token(&mut self) {
        match self.get_current_char() {
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Multiply),
            '/' => self.add_token(TokenType::Divide),

            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),

            '=' => if self.peek_next() == '=' { self.add_token(TokenType::EqualsEquals) } else { self.add_token(TokenType::Equals) }
            '!' => if self.peek_next() == '=' { self.add_token(TokenType::NotEquals) } else { self.add_token(TokenType::Not) }

            '\n' => (),
            '\t' => (),
            '\r' => (),
            ' ' => (),

            _ => Error::UnrecognizedToken.report_error(self.current_pos)
        }

        self.current_pos += 1;
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.get_current_char() != expected
        {
            return false;
        }

        self.current_pos += 1;
        return true;
    }

    fn peek_next(&self) -> char {
        let chars: Vec<char> = self.source.chars().collect();
        return chars[self.current_pos + 1];
    }

    fn get_current_char(&self) -> char {
        return self.get_source_as_chars()[self.current_pos];
    }

    fn get_source_as_chars(&self) -> Vec<char> {
        return self.source.chars().collect();
    }
}