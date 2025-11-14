(*You are to develop the functions in SML and Rust to implement binary addition utilizing pattern matching as much as possible and without using any built-in arithmetic functions. A binaryAddition function should take two integer lists of binary integers (i.e., 0s and 1s) and perform the binary addition with the doBinaryAddition function. *)
fun add 1 1 1 = [1,1]
  | add 1 1 _ = [1,0]
  | add 0 1 _ = [1]
  | add 1 0 _ = [1]
  | add _ _ _ = [0];


add 1 1 1;


fun doBinaryAddition [] [] = []
  | doBinaryAddition (l::lt,r::_) = [];






fun reverse []     = []
  | reverse (h::t) = (reverse t) @ [h];

reverse [1,2,3];

val x = [1,1] ;
x = doBinaryAddition [0,1] [1,0] ;
doBinaryAddition [0,1] [1,0] ; 








(*

Since list processing functions work best from left to right (i.e., head to tail) and binary addition is done from right to left (i.e., tail to head), you should first reverse the list. To make the computation easier, you should also convert the lists from binary integers to Boolean values (you may assume that all input values are valid).   

Binary addition can be performed on lists of unequal length. If one of the bit lists terminates before the addition is complete (as determined by the doBinaryAddition function), then the addition and carry bit should be propagated with the other bit list in the finishBinaryAdd function. 

Additionally, develop a binary subtraction function, named binarySubtraction, that takes in two lists and performs the binary subtraction. Hint: recall how binary subtraction similar to binary addition after taking the two’s compliment.
*)