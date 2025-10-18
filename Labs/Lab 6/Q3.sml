(*Blessing Abumere*)

fun square x = x*x;
fun factorial x = if x > 0 
  then x * factorial(x-1) 
  else 1;

fun pow (base,exp) =
  if exp = 0 then 1.0
  else base * pow(base,exp - 1);

fun even n = n mod 2 = 0;


fun collatz n = 
  if n = 1 then [1]
  else if n mod 2 = 0 then n :: collatz(n div 2)
  else n :: collatz(3 * n + 1);

collatz(3);
pow(2.0,5);









