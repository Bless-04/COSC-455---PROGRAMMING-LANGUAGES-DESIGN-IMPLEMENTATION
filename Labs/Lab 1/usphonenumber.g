grammar usphonenumber;

// lexical definitions
DIGIT	:	'0'..'9';

// syntax definitions
format : '1-' area_code '-' prefix '-' line_number ;
area_code: DIGIT DIGIT DIGIT;
prefix: DIGIT DIGIT DIGIT;
line_number: DIGIT DIGIT DIGIT DIGIT;
