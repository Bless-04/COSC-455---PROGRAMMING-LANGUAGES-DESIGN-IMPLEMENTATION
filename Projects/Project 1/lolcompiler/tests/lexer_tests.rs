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
        println!("{:?}", LolLexer::new("#OBTW comment #TLDR"));
    }

    #[test]
    fn test_lookup() {
        let lexer = LolLexer::new("lexer");

        assert!(
            lexer.lookup("variable"),
            "'variable' should be a valid variable name"
        );

        assert!(
            !lexer.lookup("123variable"),
            "'123variable' should NOT be a valid variable name"
        );
    }
}
