
(*that takes an integer list as a parameter and returns an integer list consisting of every other element of the parameter lis*)
fun everyother [] = [] 
  | everyother [_] = []
  | everyother (_ :: l :: lt)  = l :: everyother lt;

everyother [3, 5, 7, 11, 13, 17, 19, 29, 31, 41, 43];

[5, 11, 17, 29, 41] = everyother ([3, 5, 7, 11, 13, 17, 19, 29, 31, 41, 43]);