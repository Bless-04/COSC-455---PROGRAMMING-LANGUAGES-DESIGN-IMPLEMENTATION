use lolcompiler::compilation::lexer::LolLexer;

#[cfg(test)]
mod lexer_tests {

    fn read_file_to_string(path: &str) -> String {
        std::fs::read_to_string(path).expect(&format!("Failed to read '{}'", path))
    }

    use lolcompiler::compilation::lexer;

    use super::LolLexer; //makes it work??

    #[test]
    /// compiler only accepts .lol files.
    fn test() {
        let mut lexer = LolLexer::new("tttt");

        println!("{:?}d", lexer.tokenize());
    }
}
