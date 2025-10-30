/// Lolcompiler struct
pub struct LolCompiler {
    current_token: String,
}

/// Base Lol_Compiler
impl LolCompiler {
    pub fn new() -> Self {
        println!("Lol_Compiler Created");
        Self {
            current_token: String::new(),
        }
    }
}
/*
// Compiler impl for Lol compiler
impl Compiler for LolCompiler {
    fn compile(&mut self, source: &str) {
        todo!()
    }

    fn next_token(&mut self) -> String {
        todo!()
    }

    fn parse(&mut self) {
        todo!()
    }

    fn current_token(&self) -> String {
        todo!()
    }

    fn set_current_token(&mut self, tok: String) {
        todo!()
    }
}
*/
