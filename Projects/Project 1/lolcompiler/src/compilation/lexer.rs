// heavily inspired from Lab 5's lexical analyzer

use crate::{LexicalAnalyzer, compilation::token::Token};
use std::ops::{Index, Range, RangeFull};

#[derive(Debug)]
// Lexical Analyzer for lolcode ; first step of compiling ; 1
pub struct LolLexer<'a> {
    _text: &'a str,
    _tokens: Vec<Token<'a>>,
    _potential_token: String,
    _position: usize,
}

// implementing [index]  for LolLexer
impl<'a> Index<usize> for LolLexer<'a> {
    type Output = Token<'a>;
    fn index(&self, index: usize) -> &Self::Output {
        &self._tokens[index]
    }
}

//implementing [start..end] for LolLexer
impl<'a> Index<Range<usize>> for LolLexer<'a> {
    type Output = [Token<'a>];
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self._tokens[index]
    }
}

//implementing [..] for LolLexer
impl<'a> Index<RangeFull> for LolLexer<'a> {
    type Output = [Token<'a>];
    fn index(&self, _index: RangeFull) -> &Self::Output {
        &self._tokens[..]
    }
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

    /// looks ahead 1 char
    fn peek(&self) -> char {
        if self._position + 1 < self._text.len() {
            let c = self._text.as_bytes()[self._position + 1] as char; //works because only ascii
            c
        } else {
            '\0' //null char
        }
    }
    // readonly access to tokens
    pub fn tokens(&self) -> &Vec<Token<'a>> {
        &self._tokens
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
                self.start_potential_token();
                self._potential_token.clear();
            }
        }
    }

    /// helper function to handle potential token logic
    /// consumes characters until whitespace is found and add to token list
    fn start_potential_token(&mut self) {
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
                self._tokens.push(token);
            } else {
                //if it contains a
            }
        }
    }
}
/*


pub fn next(&mut self) -> String {
    let candidate_token = self.tokens.pop().unwrap_or_default();

    if self.lookup(&candidate_token) {
        candidate_token
    } else if self.tokens.is_empty() {
        "".to_string()
    } else {
        eprintln!(
            "A lexical error was encountered. '{}' is not a recognized token.",
            &candidate_token
        );
        std::process::exit(1);
    }
}


*/

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
        })
    }
}
