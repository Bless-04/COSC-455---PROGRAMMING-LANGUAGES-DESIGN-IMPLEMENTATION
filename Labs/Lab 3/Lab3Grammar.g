grammar Lab3Grammar;

UPPERCASE_LETTER	: 'A'..'Z';
NUMBER : '0'..'9';

LOWERCASE_LETTER : 'a'..'z';


language	: UPPERCASE_LETTER UPPERCASE_LETTER rest?;
rest	:  UPPERCASE_LETTER * NUMBER;
