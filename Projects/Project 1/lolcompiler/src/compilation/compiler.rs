use std::fs;

use crate::{
    Compiler, SyntaxAnalyzer,
    compilation::{lexer::LolLexer, parser::LolParser},
};

/// Lolcompiler struct ; last step of compiling ; 3
pub struct LolCompiler {
    _current_token: String,
}

/// Base LolCompiler
impl LolCompiler {
    pub fn new() -> Self {
        Self {
            _current_token: String::new(),
        }
    }
}

// Compiler impl for Lol compiler
impl Compiler for LolCompiler {
    fn compile(&mut self, source: &str) -> bool {
        if !source.ends_with(".lol") {
            eprintln!("Error:\t '{}' is not a .lol file.", source);
            //std::process::exit(1); exiting the process makes testing harder
            return false;
        }

        let text: String =
            fs::read_to_string(source).expect(&format!("Failed to read '{}'", source));

        // creates lexer from source text
        let lexer = LolLexer::new(text.as_str());

        // uses lexer to create parser
        let parser = LolParser::new(lexer)
            .parse_lolcode()
            .expect(&format!("Failed to parse '{}'", source));

        //println!("source: {} [{}]", source, text.len());

        true
    }

    fn next_token(&mut self) -> String {
        todo!()
    }

    fn parse(&mut self) {
        todo!()
    }

    fn current_token(&self) -> String {
        self._current_token.clone()
    }

    fn set_current_token(&mut self, token: String) {
        self._current_token = token;
    }
}
