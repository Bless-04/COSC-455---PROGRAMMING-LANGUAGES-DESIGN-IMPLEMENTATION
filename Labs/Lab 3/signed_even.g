grammar signed_even;

EVEN_NUMBER : '0' | '2' | '4' | '6' | '8';
NUMBER : '0'..'9';

language : ('+' | '-')? (NUMBER | EVEN_NUMBER)+ EVEN_NUMBER;
