// Token enum for lolcode tokens
#[derive(Debug)]
pub enum Token {
    Keyword(&'static str), // single keyword

    // literal string of something
    Literal(&'static str),

    // variable name
    Identifier(&'static str),
    // =
    Operator(&'static str),

    // <!-- -->
    Comment(&'static str),
    END,
}
