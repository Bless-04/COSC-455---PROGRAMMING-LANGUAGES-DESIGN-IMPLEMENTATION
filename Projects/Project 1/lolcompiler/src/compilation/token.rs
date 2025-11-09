// Token enum for lolcode tokens
#[derive(Debug)]
pub enum Token<'a> {
    Keyword(&'a str), // single keyword; keywords start with "#"

    VarDef(&'a str),   //any single word (A–Z, a–z, no spaces)
    VarValue(&'a str), //allowed text characters
    Address(&'a str),  //text without spaces ; should come after things like sound and video
    // variable name
    Identifier(&'a str),
    // =
    Operator(&'a str),
    // <!-- -->
    Comment(&'a str),
    END,
}
