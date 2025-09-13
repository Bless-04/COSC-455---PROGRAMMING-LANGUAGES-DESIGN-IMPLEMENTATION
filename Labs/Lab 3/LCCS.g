// Library of Congress Classification System
grammar LCCS;

UPPERCASE_LETTER	: 'A'..'Z';
LOWERCASE_LETTER : 'a'..'z';
DIGIT : '0'..'9';

call_number: topic cutter other?;
topic :part1 part2;
cutter: part3 part4?;
other: part5?  part6?;

part1: UPPERCASE_LETTER UPPERCASE_LETTER? UPPERCASE_LETTER?;
part2: DIGIT DIGIT? DIGIT? DIGIT?;
part3: '.'+ UPPERCASE_LETTER DIGIT DIGIT? DIGIT? DIGIT? ;
part4: UPPERCASE_LETTER DIGIT DIGIT?;
part5	: DIGIT DIGIT DIGIT DIGIT;
part6	:	 LOWERCASE_LETTER '.' DIGIT DIGIT? ;