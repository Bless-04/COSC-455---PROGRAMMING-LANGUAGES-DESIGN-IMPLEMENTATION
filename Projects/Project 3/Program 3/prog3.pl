% the routes
route(fresno, seattle).
route(fresno, albany).
route(fresno, boston).
route(seattle, omaha).
route(seattle, dallas).
route(omaha, albany).
route(omaha, atlanta).
route(albany, seattle).
route(albany, dallas).
route(dallas, seattle).
route(dallas, albany).
route(atlanta, boston).
route(atlanta, dallas).
route(atlanta, albany).

% Rule: flight(Start, End).
% Direct flight
flight(X, Y) :- route(X, Y).

% connects flights
flight(X, Y) :- path(X, Y, [X]).

path(X, Y, _) :- route(X, Y).
path(X, Y, Visited) :-
    route(X, Z),
    \+ member(Z, Visited), % checks if z visited
    path(Z, Y, [Z|Visited]).