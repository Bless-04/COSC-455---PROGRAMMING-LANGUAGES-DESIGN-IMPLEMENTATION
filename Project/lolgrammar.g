grammar lolgrammar;



SPACES : (' ' | '\t' | '\r' | '\n'| '\u000C')+; //makes SPACES not cause errors
WORD : ('A'..'Z' | 'a'..'z')+ ;
TEXT : (WORD | '0'..'9' | SPACES | ',' | '.' | '"' | ':' | '?' | '!' | '%' | '/' )+;

END_O : ('#OIC' | '#oic') SPACES?;
END_M : ('#MKAY' | '#mkay')  SPACES?;


language :  ('#HAI' | '#hai') SPACES? comment? head? (TEXT | paragraph | bold | sound | video | var_create | var_read   )* comment? SPACES? ('#KTHXBYE' | '#kthxbye') ;

comment : ('#OBTW'| '#obtw')  TEXT ('#TLDR'|'#tldr') SPACES? ;
head: ('#MAEK HEAD' | '#maek head')  SPACES? (comment? | title)  END_O;
title: ('#GIMMEH TITLE' | '#gimmeh title') TEXT END_M;
paragraph : ('#MAEK PARAGRAF'| '#maek paragaraf') SPACES? (TEXT | bold | italic | list| video| sound | break)* SPACES? END_O;
bold :  ('#GIMMEH BOLD' | '#gimmeh bold') TEXT END_M;
italic : ('#GIMMEH ITALICS' | '#gimmeh italics') TEXT END_M ;
list : ('#MAEK LIST' | '#maek list') SPACES? (list_item)+ END_O ;
list_item : ('#GIMMEH ITEM' | '#gimmeh item') (TEXT | bold | italic)* END_M ;
break: ('#GIMMEH NEWLINE' | '#gimmeh newline') SPACES? ;
sound : ('#GIMMEH SOUNDZ' | '#gimmeh soundz') TEXT END_M ;
video : ('#GIMMEH VIDZ' | '#gimme vidz') TEXT END_M ;
var_create : '#I HAZ' WORD '#IT IZ' TEXT END_M ;
var_read: '#LEMME SEE' WORD END_M;