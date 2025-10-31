use core::error;

use crate::parser::{compiler::LolCompiler, lexer::LolLexer};
use crate::{Compiler, SyntaxAnalyzer};

// Syntax Analyzer struct for lolcode
pub struct LolParser {}

impl LolParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl SyntaxAnalyzer for LolParser {
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
