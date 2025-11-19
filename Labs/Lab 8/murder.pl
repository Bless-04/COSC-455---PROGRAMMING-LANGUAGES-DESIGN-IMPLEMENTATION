% In Elementary Pascal Ledgard & Singer have Sherlock Holmes program
% the Analytical Engine to confirm the identity of the murderer of
% a well known art dealer at the Metropolitan Club in London.
% The murderer can be deduced from the following apparently trivial
% clues. Can you figure it out by hand?

% The murderer has brown hair.
murderer(X):-hair(X,brown). 

% Mr Holman wore a ring; Mr Pope wore a watch.
attire(mr_holman, ring). 
attire(mr_pope, watch). 

% If Sir Raymond wore tattered cuffs, then Mr Woodley wore the pincenez spectacles
attire(mr_woodley,pincenez):-attire(sir_raymond,tattered_cuffs).
% If Mr Woodley wore the tattered cuffs, then Sir Raymond wore the pincenez spectacles 
attire(sir_raymond,pincenez):-attire(mr_woodley,tattered_cuffs).

% Someone who was in room 16 was wearing tattered cuffs. 
attire(X,tattered_cuffs):-room(X,16). 

hair(X,black):-room(X,14).
hair(X,brown):-attire(X,pincenez).
hair(X,red):-attire(X,tattered_cuffs).

% Add your fact here.

% Mr Holman was in room 12; Sir Raymond in room 10; Mr Woodley in room 16.
room(mr_holman, 12).
room(sir_raymond,10).
room(mr_woodley,16).
room(X,14):-attire(X, watch).

:- nl,nl, write('The game is afoot....'), nl,nl.