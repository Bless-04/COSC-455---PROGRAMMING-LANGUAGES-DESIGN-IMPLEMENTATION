(*takes an integer list and returns a Boolean indicating whether the number 42 in the list*)
fun hitchhiker [] = false 
  | hitchhiker (42 :: _ ) = true
  | hitchhiker list = hitchhiker (tl list);


hitchhiker [4,4,5,42,7,4];


