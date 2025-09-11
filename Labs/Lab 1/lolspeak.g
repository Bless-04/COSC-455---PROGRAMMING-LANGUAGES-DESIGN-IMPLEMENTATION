grammar lolspeak;

// lexical definitions
ARTICLE : 'a' | 'teh' ;
NOUN : 'kat' | 'dawg' | 'cheezburgr' ;
VERB : 'ates' | 'lovez' | 'hatez' ;

// syntax definitions
sentence : noun_phrase VERB noun_phrase;
noun_phrase : ARTICLE NOUN;