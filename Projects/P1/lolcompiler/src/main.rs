use lolcompiler::parser::compiler::LolCompiler;
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect(); // cmd line args
    if args.len() < 2 {
        // 2 is filename
        eprintln!("Usage: lolcompiler <filepath>");

        //std::process::exit(1); uncomment when finished
    }
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Could not read file");
    let compiler = LolCompiler::new();
    println!("filepath: {} [{}]", path, contents.len());
    //let mut parser = SyntaxAnalyzer::new(&mut lexer, &mut compiler);
    /*
        parser.lolspeak();
        if !lexer.tokens.is_empty() || !compiler.current_token.is_empty() {
            eprintln!("A syntax error was encountered. Additional tokens found after the sentence.");
            std::process::exit(1);
        }
    */
}
