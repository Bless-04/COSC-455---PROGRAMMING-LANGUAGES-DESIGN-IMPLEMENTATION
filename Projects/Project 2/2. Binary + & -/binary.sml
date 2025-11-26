

(* Reverses a list *)
fun reverse [] = []
  | reverse (x::xs) = reverse xs @ [x];

fun intToBool [] = []
  | intToBool (0::xs) = false :: intToBool xs
  | intToBool (_::xs) = true :: intToBool xs (* true if not 0 *)


fun boolToInt [] = []
  | boolToInt (false::xs) = 0 :: boolToInt xs
  | boolToInt (_::xs) = 1 :: boolToInt xs;

(* length of a list *)
fun len [] = 0
  | len (_::xs) = 1 + len xs;

fun padZeros 0 list = list
  | padZeros n list = 0 :: padZeros (n-1) list;

(* Aligns two lists to the same length by padding the shorter one *)
fun alignLists l1 l2 =
    let
        val len1 = len l1
        val len2 = len l2
    in
        if len1 > len2 then (l1, padZeros (len1 - len2) l2)
        else if len2 > len1 then (padZeros (len2 - len1) l1, l2)
        else (l1, l2)
    end;

fun doBinaryAddition [] [] true = [true]
  | doBinaryAddition [] [] false = []
  | doBinaryAddition list1 [] carry = doBinaryAddition list1 [false] carry
  | doBinaryAddition [] list2 carry = doBinaryAddition [false] list2 carry
  | doBinaryAddition (x::xs) (y::ys) carry =
    let
        (* Calculate sum of bits *)
        val sumVal = (if x then 1 else 0) + (if y then 1 else 0) + (if carry then 1 else 0)
        (* Determine result bit and new carry *)
        val resultBit = (sumVal mod 2 <> 0)
        val newCarry = (sumVal >= 2)
    in
        resultBit :: doBinaryAddition xs ys newCarry
    end;

fun binaryAddition list1 list2 =
    let
        val b1 = intToBool list1
        val b2 = intToBool list2
        val r1 = reverse b1
        val r2 = reverse b2
        val sumRev = doBinaryAddition r1 r2 false
    in
        boolToInt (reverse sumRev)
    end;

(* exists to fix the [0,1,0,0] kind of issue when subtracting *)
fun trimZeros [] = [0]
  | trimZeros [0] = [0]
  | trimZeros (0::xs) = trimZeros xs
  | trimZeros xs = xs;


fun binarySubtraction list1 list2 =
    let
        (* Padding lists to make its length equal*)
        val (l1, l2) = alignLists list1 list2
        val length_target = len l1

        (* (1's complement) *)
        val b2_bool = intToBool l2
        val b2_not = map not b2_bool
        val b2_inv = boolToInt b2_not

        (* (2's complement) *)
        val twos_comp = binaryAddition b2_inv [1]

        (* Add A and 2's Complement of B *)
        val raw_result = binaryAddition l1 twos_comp
        
        (* when resul longer than the padded inputs, first bit dropped *)
    in
        if len raw_result > length_target then trimZeros (tl raw_result)
        else raw_result
    end;


[1,0,0] = binaryAddition [1,0] [1,0]; (* 2+ 2 = 4*)
[1, 0, 1, 0, 0, 1] = binaryAddition [1,1,1,1,0] [1,0,1,1];
[1, 0, 1, 1, 1, 1, 1] = binaryAddition [1,0,0,1,1,0,1]  [0,0,1,0,0,1,0  ];
[1, 1, 0, 0, 0, 1, 0] = binaryAddition [1,0,0,1,0,0,1] [0,0,1,1,0,0,1]  ;
[1, 0, 1, 1, 1, 0, 1] = binaryAddition [1,0,0,0,1,1,1] [1,0,1,1,0];

[1,0,0] =  ( binarySubtraction [1,0,0,1] [1,0,1] );
[1,0,1] =  ( binarySubtraction [1,0,0,1] [1,0,0] );
