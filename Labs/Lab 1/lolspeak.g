grammar lolspeak;

// lexical definitions
ARTICLE : 'a' | 'teh' ;
NOUN : 'kat' | 'dawg' | 'cheezburgr' ;
VERB : 'ates' | 'lovez' | 'hatez' ;
ADJECTIVE: 'fat' | 'hungry' | 'happy' | 'mean' ;
ADVERB : 'accidently' | 'quickly' | 'secretly' ;


// syntax definitions
sentence : noun_phrase ADVERB? VERB noun_phrase;
noun_phrase : ARTICLE ADJECTIVE NOUN;