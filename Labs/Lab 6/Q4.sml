(*Blessing Abumere Q4*)

fun twins list = 
  if null list then [] 
  else hd list :: hd list :: twins(tl list);



twins [1,2];






(*Blessing Abumere Q4*)

fun twins [] = []
  | twins (x::xs) = x :: x :: twins(xs) ;

(*•	A function, named convert, of type bool list -> int list that takes a list of booleans and returns the list of integer equivalents (i.e., 1 for true, 0 for false). For example, if you evaluate convert [true, true, false, true] it would return [1, 1, 0, 1].*)
fun convert [] = []
  | convert(true::rest) = 1 :: convert rest
  | convert (false::rest) = 0 :: convert rest;


(*•	A function il2rl of type int list -> real list that takes a list of integers and returns a list of the same numbers converted to type real. For example, if you evaluate il2rl [1, 2, 3] it will return [1.0, 2.0, 3.0]. Your solution must use the map curried function.*)

fun il2rl list = map (fn n => real(n)) list;

il2rl [1,2,3];

(*•	A function squarelist of type int list -> int list that takes a list of integers and returns the list of squares of those integers. For example, if you evaluate squarelist [1, 2, 3, 4] it would return [1, 4, 9, 16]. Your solution must use the map curried function.
*)
fun squarelist list = map (fn n => n * n) list;

squarelist [1,2,3];



