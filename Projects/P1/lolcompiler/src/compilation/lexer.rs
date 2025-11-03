// heavily inspired from Lab 5's lexical analyzer

use crate::{LexicalAnalyzer, compilation::token::Token};
use std::str::CharIndices;
static KEYWORDS: [&'static str; 16] = [
    "#HAI",
    "#KTHXBYE",
    "#OBTW",
    "#TLDR",
    "#MAEK HEAD",
    "#OIC",
    "#GIMMEH TITLE",
    "#MKAY",
    "#MAEK PARAGRAF",
    "#GIMMEH BOLD",
    "#GIMMEH ITALICS",
    "#MAEK LIST",
    "#GIMMEH ITEM",
    "#GIMMEH NEWLINE",
    "#GIMMEH SOUNDZ",
    "#GIMMEH VIDZ",
];

// Lexical Analyzer for lolcode ; first step of compiling ; 1
pub struct LolLexer<'a> {
    _input: CharIndices<'a>, //source text
    _tokens: Vec<Token>,
}

impl<'a> LolLexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            _input: text.char_indices(), // rust char iterator that references text
            _tokens: Vec::new(),
        }
    }
}
/*
pub fn start(&mut self) -> String {
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
}

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
        let result = self._input.next();

        match result {
            Some((p, c)) => c,
            None => '\0',
        }
    }

    fn add_char(&mut self, c: char) {
        todo!()
    }

    ///loops if something is a valid token
    ///returns true if its valid
    /// false otherwise
    fn lookup(&self, s: &str) -> bool {
        KEYWORDS.iter().any(|k| k.eq_ignore_ascii_case(s))
    }
}
