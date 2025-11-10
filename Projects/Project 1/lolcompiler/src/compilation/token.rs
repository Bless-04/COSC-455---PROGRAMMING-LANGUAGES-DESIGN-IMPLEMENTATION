// Token enum for lolcode tokens
#[derive(Debug)]
pub enum Token<'a> {
    HAI,
    KTHXBYE,
    OBTW,
    TLDR,
    MAEK,
    OIC,
    GIMMEH,
    MKAY,
    HEAD,
    TITLE,
    PARAGRAF,
    BOLD,
    ITALICS,
    LIST,
    ITEM,
    NEWLINE,
    SOUNDZ,
    VIDZ,
    IHAZ,
    ITIZ,
    LEMMESEE,

    VARDEF(&'a str),  // any single word (A–Z, a–z, no spaces)
    VARVAL(&'a str),  // allowed text characters
    TEXT(&'a str),    // plain text (letters, digits, punctuation, spaces)
    ADDRESS(&'a str), // text without spaces
    END,
}

impl Token<'_> {
    //tries to parse a &str into a Token ; returns Option<Token>
    /// idea stolen from csharps int.TryParse
    pub fn try_parse(text: &'_ str) -> Option<Token<'_>> {
        match text.to_ascii_uppercase().as_str() {
            "#HAI" => Some(Token::HAI),
            "#KTHXBYE" => Some(Token::KTHXBYE),
            "#OBTW" => Some(Token::OBTW),
            "#TLDR" => Some(Token::TLDR),
            "#MAEK" => Some(Token::MAEK),
            "#OIC" => Some(Token::OIC),
            "#GIMMEH" => Some(Token::GIMMEH),
            "#MKAY" => Some(Token::MKAY),
            "HEAD" => Some(Token::HEAD),
            "TITLE" => Some(Token::TITLE),
            "PARAGRAF" => Some(Token::PARAGRAF),
            "BOLD" => Some(Token::BOLD),
            "ITALICS" => Some(Token::ITALICS),
            "LIST" => Some(Token::LIST),
            "ITEM" => Some(Token::ITEM),
            "NEWLINE" => Some(Token::NEWLINE),
            "SOUNDZ" => Some(Token::SOUNDZ),
            "VIDZ" => Some(Token::VIDZ),
            "#I HAZ" => Some(Token::IHAZ),
            "#IT IZ" => Some(Token::ITIZ),
            "#LEMME SEE" => Some(Token::LEMMESEE),
            _ => None,
        }
    }
}
