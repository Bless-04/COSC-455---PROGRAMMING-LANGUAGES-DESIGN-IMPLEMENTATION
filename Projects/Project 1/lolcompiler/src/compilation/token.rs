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
    VARDEF(&'a str), //any single word (A–Z, a–z, no spaces)
    VARVAL(&'a str), //allowed text characters

    TEXT(&'a str),    //plain text (letters, digits, punctuation, spaces)
    ADDRESS(&'a str), //text without spaces ; should come after things like sound and video

    END,
}
