use lolcompiler::compilation::lexer::LolLexer;

#[cfg(test)]
mod lexer_tests {
    use super::LolLexer;
    fn read_file_to_string(path: &str) -> String {
        std::fs::read_to_string(path).expect(&format!("Failed to read '{}'", path))
    }

    #[test]
    /// compiler only accepts .lol files.
    fn test() {
        let mut lexer = LolLexer::new("tttt");

        println!("{:?}", lexer);
    }
}
