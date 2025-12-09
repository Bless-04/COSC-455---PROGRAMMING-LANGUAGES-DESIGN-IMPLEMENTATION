% Course database.

course(cosc455,
       location(yr7800, 204),
       time(t, 1100),
       instructor(dehlinger)).
course(cosc455,
       location(yr7800, 402),
       time(r, 1100),
       instructor(dehlinger)).

course(cosc612,
       location(yr7800, 125),
       time(w, 1900),
       instructor(sai)).

course(cosc465,
       location(yr7800, 202),
       time(t, 1230),
       instructor(mallik)).

course(cosc465,
       location(yr7800, 202),
       time(r, 1230),
       instructor(mallik)).

course(cosc439,
       location(yr7800, 401),
       time(m, 1900),
       instructor(sajid)).

course(cosc578,
       location(yr7800, 202),
       time(m, 1900),
       instructor(zimand)).

course(cosc175,
       location(yr7800, 205),
       time(t, 1100),
       instructor(conover)).

course(cosc175,
       location(yr7800, 205),
       time(r, 1100),
       instructor(conover)).

course(cosc236,
       location(yr7800, 204),
       time(m, 1100),
       instructor(chakraborty)).

course(cosc236,
       location(yr7800, 204),
       time(w, 1100),
       instructor(chakraborty)).

course(cosc336,
       location(yr7800, 402),
       time(t, 930),
       instructor(tang)).

course(cosc336,
       location(yr7800, 402),
       time(r, 930),
       instructor(tang)).

course(cosc417,
       location(yr7800, 401),
       time(m, 1400),
       instructor(zimand)).

course(cosc417,
       location(yr7800, 401),
       time(w, 1400),
       instructor(zimand)).

course(cosc418,
       location(yr7800, 202),
       time(t, 1400),
       instructor(wang)).

course(cosc418,
       location(yr7800, 202),
       time(r, 1400),
       instructor(wang)).

course(cosc412,
       location(yr7800, 125),
       time(t, 1900),
       instructor(deng)).

course(cosc412,
       location(yr7800, 125),
       time(r, 1900),
       instructor(deng)).




% blessing
       course(cosc457,
       location(yr7800, 301),
       time(m, 1530),
       instructor(kulkarni)).

         course(math274,
       location(yr7800, 210),
       time(r, 1800),
       instructor(riggs)).

        course(math263,
       location(yr7800, 210),
       time(r, 1445),
       instructor(xhane)).


is_teaching(Last, time(Day, Hour)) :-
    course(_, _, time(Day, Hour), instructor(Last)).

is_instructor(Last, Course) :-
    course(Course, _, _, instructor(Last)).

is_busy(Last, Day, Hour, Location) :-
    course(_, Location, time(Day, Hour), instructor(Last)).

cannot_meet(time(Day, Hour), Inst1, Inst2) :-
    is_teaching(Inst1, time(Day, Hour));
    is_teaching(Inst2, time(Day, Hour)).

schedule_conflict(C1, C2) :-
    course(C1, _, time(D, H), _),
    course(C2, _, time(D, H), _),
    C1 \= C2.
