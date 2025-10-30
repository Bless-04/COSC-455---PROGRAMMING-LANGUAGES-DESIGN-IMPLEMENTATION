use lol_compiler::parser::compiler::LolCompiler;

#[cfg(test)]
mod compiler_tests {
    use super::LolCompiler; //makes it work??
    #[test]
    fn test_new() {
        let compiler = LolCompiler::new();
        assert!(false);
    }

    #[test]
    fn test_compile() {
        let mut compiler = LolCompiler::new();

        assert!(false);
    }

    #[test]
    fn test_next_token() {
        let mut compiler = LolCompiler::new();

        assert!(false);
    }

    #[test]
    fn test_parse() {
        let mut compiler = LolCompiler::new();

        assert!(false);
    }

    #[test]
    fn test_current_token() {
        let new_token: String = "test".to_string();
        let mut compiler = LolCompiler::new();

        assert!(false);
    }

    #[test]
    fn test_set_current_token() {
        let mut compiler = LolCompiler::new();

        assert!(false);
    }
}
