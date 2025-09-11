grammar usps;

// lexical definitions
DIGIT	:	'0'..'9';
LETTER	:	'A'..'Z';
CODE	:	'US' | 'KR' | 'ZH';


// syntax definititions
tracking_number	:	letter_part number_part country_code ;
letter_part	: 	LETTER LETTER ;
number_part	:	DIGIT DIGIT DIGIT DIGIT DIGIT DIGIT DIGIT DIGIT DIGIT ; 
country_code	:	CODE;



