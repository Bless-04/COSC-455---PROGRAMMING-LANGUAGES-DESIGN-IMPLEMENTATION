translation(ling, zero).
translation(yi, one).
translation(er, two).
translation(san, three).
translation(si, four).
translation(wu, five).
translation(liu, six).
translation(qi, seven).
translation(ba, eight).
translation(jiu, nine).
translation(shi, ten).

% Base case
translate([], []).

% L1 is list 1, LT1 is list 1 tail etc
translate([L1|LT1], [L2|LT2]) :-
    translation(L1, L2),
    translate(LT1, LT2).

