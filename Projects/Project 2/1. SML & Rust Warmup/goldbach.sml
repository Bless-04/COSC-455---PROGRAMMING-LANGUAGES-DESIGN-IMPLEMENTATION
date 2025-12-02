(*true if prime*)
fun isPrime n =
    let
        fun checkDivisor d =
            if d * d > n then true
            else if n mod d = 0 then false
            else checkDivisor (d + 1)
    in
        if n < 2 then false
        else checkDivisor 2
    end;

fun goldbach n =
    if n <= 2 orelse n mod 2 <> 0 then
        print "Error: Integer must be even and greater than 2. "
    else
        let
            (* Searching for a prime pair*)
            fun findPair i =
                if isPrime i andalso isPrime (n - i) then
                    print (Int.toString i ^ " + " ^ Int.toString (n - i) ^ " = " ^ Int.toString n ^ "\n")
                else
                    findPair (i + 1)
        in
            findPair 2
        end;

goldbach 28;(* 5 + 23 = 28 *)
goldbach 11;(*Error if working *)