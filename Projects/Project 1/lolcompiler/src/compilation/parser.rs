use crate::{SyntaxAnalyzer, compilation::lexer::LolLexer};
use std::collections::HashMap;

use crate::{
    SyntaxAnalyzer,
    compilation::{lexer::LolLexer, token::Token},
};

// Syntax Analyzer struct for lolcode ; middle step of compiling ; 2
pub struct LolParser<'a> {
    _lexer: LolLexer<'a>,
    _html: String,
    _variables: HashMap<String, String>, // vardef, varval
}

impl<'a> LolParser<'a> {
    pub fn new(token_source: LolLexer<'a>) -> Self {
    pub fn new(mut token_source: LolLexer<'a>) -> Self {
        token_source.start(); // Tokenize the input
        Self {
            _lexer: token_source,
            _html: String::new(),
            _variables: HashMap::new(),
        }
    }
}

impl<'a> SyntaxAnalyzer for LolParser<'a> {
    //HAI <comments> < head> <body> KTHXBYE
    fn parse_lolcode(&mut self) -> Result<(), String> {
        todo!()
    }

    fn parse_head(&mut self) -> Result<(), String> {
        todo!()
    }

    fn parse_title(&mut self) -> Result<(), String> {
        todo!()
    }

    fn parse_comment(&mut self) -> Result<(), String> {
        todo!()
    }

    fn parse_body(&mut self) -> Result<(), String> {
        todo!()
    }

    fn parse_paragraph(&mut self) -> Result<(), String> {
        todo!()
    }

    fn parse_inner_paragraph(&mut self) -> Result<(), String> {
        todo!()
    }

    fn parse_inner_text(&mut self) -> Result<(), String> {
        todo!()
    }

    fn parse_variable_define(&mut self) -> Result<(), String> {
        todo!()
    }

    fn parse_variable_use(&mut self) -> Result<(), String> {
        todo!()
    }

    fn parse_bold(&mut self) -> Result<(), String> {
        todo!()
    }

    fn parse_italics(&mut self) -> Result<(), String> {
        todo!()
    }

    fn parse_list(&mut self) -> Result<(), String> {
        todo!()
    }

    fn parse_list_items(&mut self) -> Result<(), String> {
        todo!()
    }

    fn parse_inner_list(&mut self) -> Result<(), String> {
        todo!()
    }

    fn parse_audio(&mut self) -> Result<(), String> {
        todo!()
    }

    fn parse_video(&mut self) -> Result<(), String> {
        todo!()
    }

    fn parse_newline(&mut self) -> Result<(), String> {
        todo!()
    }

    fn parse_text(&mut self) -> Result<(), String> {
        todo!()
    }
}
