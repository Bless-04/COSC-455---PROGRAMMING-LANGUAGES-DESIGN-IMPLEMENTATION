affair(boddy, green).
affair(scarlett, boddy).
married(plum, green).
rich(boddy).
greedy(mustard).

spouse(X, Y) :- married(X, Y).

% fixes infinity issue
spouse(X, Y) :- married(Y, X).


motive(Person, greed) :-
    greedy(Person),
    \+ rich(Person),
    rich(boddy).

% Person hates victim when spouse cheated on them
motive(Person, hatred) :-
    spouse(Person, Spouse),
    affair(boddy, Spouse).

% Suspect Rule
suspect(Person) :- motive(Person, _).


% makes the suspect unique:
% rich(mustard).