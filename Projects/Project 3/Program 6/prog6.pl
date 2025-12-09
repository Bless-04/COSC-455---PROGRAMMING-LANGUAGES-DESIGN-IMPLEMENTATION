prog6(HomerPath) :-
    % 4 positions
    HomerPath = [_, _, _, _],

    % Gifts: dress, sax_book, slingshot, pacifier
    % Stores: leftorium, sprawl_mart, try_n_save, king_toots

    %   bought the saxophone book at King Toots
    member(stop(sax_book, king_toots), HomerPath),

    %   Leftorium was  second stop
    nth1(2, HomerPath, stop(_, leftorium)),

    %  Two stops after leaving Try-N-Save, he bought the pacifier
    % This implies Try-N-Save is index 1 and Pacifier is index 3, OR Try-N-Save is 2 and Pacifier is 4.
    (
        (nth1(1, HomerPath, stop(_, try_n_save)), nth1(3, HomerPath, stop(pacifier, _)));
        (nth1(2, HomerPath, stop(_, try_n_save)), nth1(4, HomerPath, stop(pacifier, _)))
    ),

    % Filing in remaining Gifts
    member(stop(dress, _), HomerPath),
    member(stop(slingshot, _), HomerPath),

    % Filling in remaining Stores
    member(stop(_, sprawl_mart), HomerPath),
    % (Leftorium, King Toots, Try-N-Save handled)

    %  The store visited after buying the slingshot isnt Spral
    next_to(stop(slingshot, _), stop(_, AfterSlingshotStore), HomerPath),
    AfterSlingshotStore \= sprawl_mart.

% helps check adjacency
next_to(A, B, [A, B | _]).
next_to(A, B, [_ | Rest]) :- next_to(A, B, Rest).