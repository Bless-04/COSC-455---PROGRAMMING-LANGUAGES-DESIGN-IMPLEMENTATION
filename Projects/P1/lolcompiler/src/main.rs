use lolcompiler::parser::{
    compiler::LolCompiler, lexer::LolLexer, syntax_analyzer::SyntaxAnalyzer,
};

use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect(); // cmd line args
    if args.len() < 2 {
        // 2 is filename
        eprintln!("Usage: lolcompiler <filepath>");
        //std::process::exit(1);
    }
    let path = &args[1];

    if !path.ends_with(".lol") {
        eprintln!("Error: The file given is not a .lol file.");
        //  std::process::exit(1);
    }

    let text = fs::read_to_string(path).expect(&format!("Failed to read '{}'", path));
    let compiler = LolCompiler::new();
    println!("filepath: {} [{}]", path, text.len());
    //let mut parser = SyntaxAnalyzer::new(&mut lexer, &mut compiler);
    /*

        if !lexer.tokens.is_empty() || !compiler.current_token.is_empty() {
            eprintln!("A syntax error was encountered. Additional tokens found after the sentence.");
            std::process::exit(1);
        }
    */
}
