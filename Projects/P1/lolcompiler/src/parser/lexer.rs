// heavily inspired from Lab 5's lexical analyzer

use crate::LexicalAnalyzer;

// Lexical Analyzer for lolcode
pub struct LolLexer {
    tokens: Vec<String>,
}

impl LolLexer {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
        /*
        //puts a bunch of provided words into words so we can use them
        let mut tokens: Vec<String> = input
            .split(' ') // wrong
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        tokens.reverse();

        Self {
            tokens,

                adverbs: vec!["accidently".into(), "quickly".into(), "secretly".into()],
                articles: vec!["a".into(), "teh".into()],
                adjectives: vec!["fat".into(), "hungry".into(), "happy".into(), "mean".into()],
                verbs: vec!["lovez".into(), "hatez".into(), "ates".into()],
                nouns: vec!["dawg".into(), "kat".into(), "rat".into()],

        }
        */
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
}

impl LexicalAnalyzer for LolLexer {
    fn get_char(&mut self) -> char {
        todo!()
    }

    fn add_char(&mut self, c: char) {
        todo!()
    }

    ///loops if something is a valid token
    ///returns true if its valid
    /// false otherwise
    fn lookup(&self, s: &str) -> bool {
        todo!()
    }
}
