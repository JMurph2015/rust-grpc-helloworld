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

## Things to note
* There is a custom build script, it has project-specific verbiage
in there, so that will depend on your setup.
* The nonsense around the ``.to_owned()`` on the String is magic to me, 
I don't know what's going on; I just know that the compiler was angry.
* I used ``antidote::Mutex`` throughout because poisonable mutexes are
annoying and don't provide additional benefit 99% of the time.  If you
use the ``std::sync::Mutex``, you'll need to do error handling/unwrapping
on each ``.lock()`` call.