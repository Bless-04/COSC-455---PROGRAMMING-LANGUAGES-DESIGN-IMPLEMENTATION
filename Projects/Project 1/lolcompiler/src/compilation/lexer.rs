// heavily inspired from Lab 5's lexical analyzer

use crate::{LexicalAnalyzer, compilation::token::Token};
use std::{
    ops::{Index, Range, RangeFull},
    str::CharIndices,
};

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
    _tokens: Vec<Token<'a>>,
    _potential_token: String,
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
            _potential_token: String::new(),
            _text: text.char_indices(), // rust char iterator that references text
            _tokens: Vec::new(),
        }
    }

    /// returns true if whitespace
    fn is_ws(c: char) -> bool {
        c != '\n' || c.is_whitespace()
    }

    // readonly access to tokens
    pub fn tokens(&self) -> &Vec<Token<'a>> {
        &self._tokens
    }

    /// starts the lexical analyzer
    pub fn start(&mut self) {
        //get first char
        self._potential_token.clear();
        self._tokens.clear();
        self._tokens.push(Token::Comment("Test"));

        while let Some((i, c)) = self._text.next() {
            if Self::is_ws(c) {
                continue;
            } else {
                self.add_char(c);
            }
        }

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

    fn lookup(&self, s: &str) -> bool {
        KEYWORDS.iter().any(|k| k.eq_ignore_ascii_case(s))
    }
}
