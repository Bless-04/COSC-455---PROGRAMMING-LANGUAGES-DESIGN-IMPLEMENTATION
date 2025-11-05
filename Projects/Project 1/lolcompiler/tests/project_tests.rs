use lolcompiler::compilation::compiler::LolCompiler;

#[cfg(test)]
mod project_tests {
    use lolcompiler::Compiler;

    use super::LolCompiler; //makes it work??

    #[test]
    /// compiler only accepts .lol files.
    fn test1_ext() {
        let mut compiler = LolCompiler::new();
        assert!(
            compiler.compile(".lol/Test1.txt"),
            "Compiler should only accept .lol files"
        );
    }
}
