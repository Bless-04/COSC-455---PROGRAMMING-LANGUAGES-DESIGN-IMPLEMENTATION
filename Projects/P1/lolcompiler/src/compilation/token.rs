// Token enum for lolcode tokens
#[derive(Debug)]
pub enum Token {
    Keyword(&'static str), // single keyword; keywords start with "#"

    VarDef(&'static str),   //any single word (A–Z, a–z, no spaces)
    VarValue(&'static str), //allowed text characters
    Address(&'static str),  //text without spaces ; should come after things like sound and video

    // variable name
    Identifier(&'static str),
    // =
    Operator(&'static str),

    // <!-- -->
    Comment(&'static str),
    END,
}
