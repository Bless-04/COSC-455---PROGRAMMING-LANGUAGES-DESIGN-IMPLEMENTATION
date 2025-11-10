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
        self._position = 0; // Reset position for tokenization

        while self._position < self._text.len() {
            let c = self.get_char();
            if c.is_whitespace() {
                continue;
            } else if c == '#' || c.is_ascii_alphanumeric() {
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

        self._tokens.push(Token::End);
        self._position = 0; //reset position for syntax analyzer to read tokens
    }

    pub fn peek_token(&self) -> Option<&Token<'a>> {
        if self._position < self._tokens.len() {
            Some(&self._tokens[self._position])
        } else {
            None
        }
    }

    pub fn next_token(&mut self) -> Option<&Token<'a>> {
        if self._position < self._tokens.len() {
            let token = &self._tokens[self._position];
            self._position += 1;
            Some(token)
        } else {
            None
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
                // if the last token is video or soundz, then the current token is an Address
                let last_token = self._tokens.last();
                if last_token.is_none() {
                    return Err(format!(
                        "Syntax Error: The first token should atleast be a keyword but was '{}'",
                        current_str()
                    ));
                }
                match last_token.unwrap() {
                    Token::SOUNDZ | Token::VIDZ => {
                        //address case
                        return Ok(Token::Address(current_str()));
                    }
                    Token::HAZ => {
                        return Ok(Token::VarDef(current_str()));
                    }
                    Token::IZ => {
                        return Ok(Token::VarVal(current_str()));
                    }
                    _ => {
                        return Ok(Token::Text(current_str()));
                    }
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
