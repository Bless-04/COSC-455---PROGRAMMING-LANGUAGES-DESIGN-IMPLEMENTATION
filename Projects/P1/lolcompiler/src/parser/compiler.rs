use crate::{
    Compiler, SyntaxAnalyzer,
    parser::{lexer::LolLexer, parser::LolParser},
};
use std::fs;
/// Lolcompiler struct
pub struct LolCompiler {
    parser: LolParser,
    lexer: LolLexer,
    current_token: String,
}

/// Base LolCompiler
impl LolCompiler {
    pub fn new() -> Self {
        Self {
            parser: LolParser::new(),
            lexer: LolLexer::new(),
            current_token: String::new(),
        }
    }
}

// Compiler impl for Lol compiler
impl Compiler for LolCompiler {
    fn compile(&mut self, source: &str) {
        if !source.ends_with(".lol") {
            eprintln!("Error: The file given is not a .lol file.");
            //  std::process::exit(1);
        }

        let text = fs::read_to_string(source).expect(&format!("Failed to read '{}'", source));
        println!("source: {} [{}]", source, text.len());

        todo!()
    }

    fn next_token(&mut self) -> String {
        todo!()
    }

    fn parse(&mut self) {
        self.parser.parse_lolcode();
        todo!()
    }

    fn current_token(&self) -> String {
        self.current_token.clone()
    }

    fn set_current_token(&mut self, token: String) {
        self.current_token = token;
    }
}
