# ActorSystemPrototype

This is a actor framework prototype, I want to use to implement an actor based interpreted programming language.

The initial goal is not to create a new actor framework, but to create a programming language that is tuned for
debugging and working in a repl. Thats why the initial implementation of the "actor system" is rather rudimentary.

In the long run, I hope this will evolve into a usable language, but for now its just gonna be an experiment.

## Planned Features
### Script Language
The language will be custom made and will get compile to bytecode. The language will probably be a simple C
like programming language, with some features added (actor spawning, sending messages, pattern matching).

I'm also playing with the thought to remove certain features from the language, to keep the code inside the actor
not turing complete. This would enable advanced analysis on the code inside an actor, but still allow the
system as a whole to work in a turing complete fashion. Not sure if I include this here or if I'll focus on the rest
first.

### Actor System
Each actor consist of its internal state,
its mailbox and its bytecode. This should make it possible to move actors between network nodes,
by sending it's bytecode and internal state over the network and create a new actor at the other end.

### REPL
The whole language should enable a repl based workflow, where it's possible to add, modify and inspect code at runtime.

It should be possible to load the different actors in the system and inspect their local state, as well as send them
messages at runtime.

### Cluster Mode
The interpreter should be able to connect to other interpreters on the network, in order to send messages to them.

### Remote Debugging
The interpreter should have remote debugging capabilities.
It should be possible to inspect each actor inside the cluster.

Because it should be quite simple to move the actors around, it should also be possible to move the actor that you
want to debug to your local machine in order to make it simpler to inspect and debug.