use std::fs;

use crate::{
    Compiler, SyntaxAnalyzer,
    compilation::{lexer::LolLexer, parser::LolParser},
};

/// Lolcompiler struct ; last step of compiling ;
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

        // uses lexer to create parser (parser will tokenize in new())
        let mut parser = LolParser::new(lexer);

        // Parse the lolcode
        match parser.parse_lolcode() {
            Ok(()) => {
                // Generate output filename by replacing .lol with .html
                let output_file = if source.ends_with(".lol") {
                    source[..source.len() - 4].to_string() + ".html"
                } else {
                    source.to_string() + ".html"
                };

                // Write HTML to file
                match fs::write(&output_file, parser.get_html()) {
                    Ok(_) => {
                        println!("Compilation successful. Output written to: {}", output_file);
                        true
                    }
                    Err(e) => {
                        eprintln!("Error writing HTML file: {}", e);
                        false
                    }
                }
            }
            Err(e) => {
                eprintln!("Parse error: {}", e);
                false
            }
        }
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
