use lolcompiler::{Compiler, compilation::compiler::LolCompiler};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); // cmd line args
    if args.len() < 2 {
        // 2 is filename
        eprintln!("Usage: lolcompiler <filepath>");
        std::process::exit(1);
    }

    LolCompiler::new().compile(&args[1]);
}
