grammar date;

 // lexical definitions
DIGIT : '0' .. '9';

// syntax definitions
date : month '/' day '/' year ;
year : DIGIT DIGIT DIGIT DIGIT ;

month : DIGIT DIGIT ;
day : DIGIT DIGIT ;
