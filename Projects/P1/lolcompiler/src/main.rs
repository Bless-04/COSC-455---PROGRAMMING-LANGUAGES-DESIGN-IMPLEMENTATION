use lolcompiler::{Compiler, parser::compiler::LolCompiler};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); // cmd line args
    if args.len() < 2 {
        // 2 is filename
        eprintln!("Usage: lolcompiler <filepath>");
        //std::process::exit(1);
    }
    let path = &args[1];
    let mut compiler = LolCompiler::new();
    compiler.compile(path);

    //let mut parser = SyntaxAnalyzer::new(&mut lexer, &mut compiler);
    /*

        if !lexer.tokens.is_empty() || !compiler.current_token.is_empty() {
            eprintln!("A syntax error was encountered. Additional tokens found after the sentence.");
            std::process::exit(1);
        }
    */
}
