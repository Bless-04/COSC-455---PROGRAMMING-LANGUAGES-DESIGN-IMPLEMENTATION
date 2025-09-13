grammar html;

HEAD_BEGIN 	: '<html>' | '<HTML>';
HEAD_END 	: '</html>' | '</HTML>';
P_BEGIN	: '<p>' | '<P>';
P_END	: '</p>' | '</P>';
B_BEGIN	: '<b>' | '<B>';
B_END	: '</b>' | '</B>';
TEXT : ('A'..'Z' | 'a'..'z' | '0'..'9' | '\t' | ' ' | '\r' | '\n'| '\u000C')*;


language : HEAD_BEGIN (TEXT | paragraph | bold)* HEAD_END;

paragraph : P_BEGIN (TEXT | bold)+  P_END;
bold : B_BEGIN TEXT B_END;
