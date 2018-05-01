# gRPC Hello World Example
## Purpose
This was made to prove to myself that the framework could handle what 
I needed it for.  There were two primary requirements for my needs:

1. A sane, stateful, and race-safe server
2. Ability to retain a reference to the server state after 
starting the server (to enable long running threads acting on the state)

From this project I found that gRPC is probably the right call for my
needs, though it is a bit clunkier than tarpc presently.

## Usage
Just run ``cargo run`` in the root directory, and it should take care of
the rest.  Use ``Ctrl-C`` to stop it.  There isn't much to see other than
the transition at T+10 seconds when the greeting switches.  I tried to make
this code as simple as possible so that it would be legible to relative
noobs like me.