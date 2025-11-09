use lolcompiler::compilation::lexer::LolLexer;

#[cfg(test)]
mod lexer_tests {
    use lolcompiler::LexicalAnalyzer;

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

    #[test]
    fn test_lookup() {
        let lexer = LolLexer::new("tttt");

        assert_eq!(lexer.lookup("#HAI"), true);
        assert_eq!(lexer.lookup("#hAi"), true); //case insensitive 1
        assert_eq!(lexer.lookup("#hai"), true); //case insensitive 2
        assert_eq!(lexer.lookup("random text"), false);
    }
}
