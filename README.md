# Magic

A digital recreation of Magic: The Gathering's Alpha set.

Magic is an incredibly complicated game.  The challenge of recreating
it is allowing the interaction of cards with unique effects without
having to recreate large parts of the code base.  I also want the game
to be pretty performant even with large numbers of permanents on the
battlefield.  I chose Rust because it has high performance in single
threaded operations and allows me to easily convert the code base to a
concurrent style.  Because of this requirement, the code base must be
fully multi threaded.

I plan to document most things as to how they relate to the game
itself but not the logic of how that is done.  Since I don't plan to
write this documentation, the code must be clean and understandable.
I generally achieve this through heavy usage of the facade pattern and
splitting up functions into small blocks that are easy to reason
about.  This also facilitates code re-use as the code is already split
up into the small chunks.
