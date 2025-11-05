// heavily inspired from Lab 5's lexical analyzer

use crate::{LexicalAnalyzer, compilation::token::Token};
use std::str::CharIndices;

static KEYWORDS: [&'static str; 21] = [
    "#HAI",
    "#KTHXBYE",
    "#OBTW",
    "#TLDR",
    "#MAEK",
    "#OIC",
    "#GIMMEH",
    "#MKAY",
    "HEAD",
    "TITLE",
    "PARAGRAF",
    "BOLD",
    "ITALICS",
    "LIST",
    "ITEM",
    "NEWLINE",
    "SOUNDZ",
    "VIDZ",
    "#I HAZ",
    "#IT IZ",
    "#LEMME SEE",
];

#[derive(Debug)]
// Lexical Analyzer for lolcode ; first step of compiling ; 1
pub struct LolLexer<'a> {
    _text: CharIndices<'a>, //source text
    _tokens: Vec<Token>,
    _potential_token: String,
}

impl<'a> LolLexer<'a> {
    fn is_keyword(s: &str) -> bool {
        KEYWORDS.iter().any(|k| k.eq_ignore_ascii_case(s))
    }

    fn is_ws(c: char) -> bool {
        c != '\n' || c.is_whitespace()
    }
    pub fn new(text: &'a str) -> Self {
        Self {
            _potential_token: String::new(),
            _text: text.char_indices(), // rust char iterator that references text
            _tokens: Vec::new(),
        }
    }

    pub fn tokens(&self) -> &Vec<Token> {
        &self._tokens
    }
    pub fn tokenize(&mut self) {

        /*
        let candidate_token = self.tokens.pop().unwrap_or_default();

        if self.lookup(&candidate_token) {
            candidate_token
        } else if !candidate_token.is_empty() {
            eprintln!(
                "A lexical error was encountered. '{}' is not a recognized token.",
                candidate_token
            );
            std::process::exit(1);
        } else {
            eprintln!("A user error was encountered. The provided sentence is empty.");
            std::process::exit(1);
        }
        */
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

pub fn is_a_article(&self, word: &str) -> bool {
    self.articles.iter().any(|a| a == word)
}
*/

impl LexicalAnalyzer for LolLexer<'_> {
    fn get_char(&mut self) -> char {
        let result = self._text.next();

        match result {
            Some((_, c)) => c,
            None => '\0',
        }
        //todo: terminate program if exhausted
    }

    fn add_char(&mut self, c: char) {
        self._potential_token.push(c);
    }

    ///loops if something is a valid token
    ///returns true if its valid
    /// false otherwise
    fn lookup(&self, s: &str) -> bool {
        Self::is_keyword(s)
    }
}
