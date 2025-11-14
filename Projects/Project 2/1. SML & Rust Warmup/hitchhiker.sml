(*takes an integer list and returns a Boolean indicating whether the number 42 in the list*)
fun hitchhiker [] = false 
  | hitchhiker (42 :: _ ) = true
  | hitchhiker list = hitchhiker (tl list);

hitchhiker [1,2,3,4,42];
hitchhiker [0,2,3,4,1];
hitchhiker [42,2,3,4,1];
