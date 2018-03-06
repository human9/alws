ALWS will keep a record of all your progress.
It's like having a videogame mission log for your life.
You can create new missions from scratch, or link to one or more previous missions.
Missions can be appended to constantly until they are closed. Each revision will add a new date node.
You can then view all previous missions, current missions, and assign priorities.

INTERNALS PROPOSALS
We need to have access to all current missions.

PLAIN TEXT
 - each mission is an expandable structure
 - a ?complete tag will show false for any still open
 - dates are listed for each mission
pros:
+ simple to implement
cons:
- have to read/write entire file every time

