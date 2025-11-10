// heavily inspired from Lab 5's lexical analyzer

use crate::{LexicalAnalyzer, compilation::token::Token};

#[derive(Debug)]
// Lexical Analyzer for lolcode ; first step of compiling ; 1
pub struct LolLexer<'a> {
    _text: &'a str,
    _tokens: Vec<Token<'a>>,
    _potential_token: String,
    _position: usize,
}

impl<'a> LolLexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            _position: 0,
            _potential_token: String::new(),
            _text: text,
            _tokens: Vec::new(),
        }
    }

    /// starts the lexical analyzer
    pub fn start(&mut self) {
        self._potential_token.clear();
        self._tokens.clear();

        while self._position < self._text.len() {
            let c = self.get_char();
            if c.is_whitespace() {
            } else if c == '#' {
                //potential token start
                self._potential_token.push(c);
                let token = self.start_potential_token();
                match token {
                    Ok(t) => self._tokens.push(t),
                    Err(e) => panic!("{}", e),
                }
                self._potential_token.clear();
            }
        }
    }

    /// helper function to handle potential token logic
    /// consumes characters until whitespace is found and add to token list
    fn start_potential_token(&mut self) -> Result<Token<'a>, String> {
        let mut end_pos = self._position;
        while self._position < self._text.len() {
            let c = self.get_char();
            end_pos += 1;
            if c.is_whitespace() {
                break;
            }
            self._potential_token.push(c);
        }

        //anon function to get &str from current potential token
        let current_str =
            || &self._text[self._position - self._potential_token.len() - 1..end_pos - 1];

        let s: &str = self._potential_token.as_str();
        //checks if valid token
        if self.lookup(s) {
            // case when token is one of the keywords
            if let Some(token) = Token::try_parse(s) {
                return Ok(token);
            } else {
                // otherwise it's a VarDef or VarVal
                //check if it has any non-letter characters
                if s.chars().all(|c| c.is_ascii_alphabetic()) {
                    return Ok(Token::VarDef(current_str()));
                } else {
                    return Ok(Token::VarVal(current_str()));
                }
            }
            //if it has a digit i
        }

        Err(format!("Lexical Error: Invalid token '{}'", current_str()))
    }
}

impl LexicalAnalyzer for LolLexer<'_> {
    fn get_char(&mut self) -> char {
        if self._position < self._text.len() {
            let c = self._text.as_bytes()[self._position] as char; //works because only ascii
            self._position += 1;
            c
        } else {
            panic!("Input exhausted");
        }
    }

    fn add_char(&mut self, c: char) {
        self._potential_token.push(c);
    }

    fn lookup(&self, s: &str) -> bool {
        //loop for A-Z, a-z, 0-9, commas, period, quotes, colons, question marks, underscores and forward slashes
        s.chars().all(|c| {
            c.is_ascii_alphanumeric()
                || c == ','
                || c == '.'
                || c == '\''
                || c == ':'
                || c == '?'
                || c == '_'
                || c == '/'
                || c == '#'
        })
    }
}

//for easier testing
use std::ops::Deref;

impl<'a> Deref for LolLexer<'a> {
    type Target = [Token<'a>];

    fn deref(&self) -> &Self::Target {
        &self._tokens
    }
}
