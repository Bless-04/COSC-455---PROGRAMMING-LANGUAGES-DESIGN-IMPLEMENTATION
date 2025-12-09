(*takes an integer list and returns a Boolean indicating whether the number 42 in the list*)
fun hitchhiker [] = false 
  | hitchhiker (42 :: _ ) = true
  | hitchhiker list = hitchhiker (tl list);

(*everything should be true*)
true = hitchhiker [1,2,3,4,42];
false = hitchhiker [0,2,3,4,1];
true = hitchhiker [42,2,3,4,1];
